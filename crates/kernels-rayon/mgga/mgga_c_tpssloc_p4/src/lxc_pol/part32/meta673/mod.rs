//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta673(t24682: f64, t460: f64, t95413: f64, t1409: f64, t461: f64, t1009: f64, t7324: f64, t24722: f64, t15548: f64, t24733: f64, t27598: f64, t3535: f64, t2132: f64, t24746: f64, t3545: f64, t8020: f64, t1202: f64, t27603: f64, t24736: f64, t4993: f64, t15486: f64, t7345: f64, t27599: f64, t3572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95415, t95420, t95422, t95424, t95435, t95440) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2107(t24682, t460, t95413, t1409, t461, t1009, t7324, t24722, t15548, t24733, t27598, t3535);
        let (t95446, t95450, t95452, t95456, t95459, t95463) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2108(t2132, t24746, t95413, t3545, t8020, t1202, t27603, t24736, t4993, t15486, t7345, t27599, t3572);
    (t95415, t95420, t95422, t95424, t95435, t95440, t95446, t95450, t95452, t95456, t95459, t95463)
}
