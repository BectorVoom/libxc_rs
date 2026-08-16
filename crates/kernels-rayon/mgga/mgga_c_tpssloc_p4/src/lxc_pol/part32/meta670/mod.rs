//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta670(t15437: f64, t24728: f64, t24732: f64, t4965: f64, t7344: f64, t1184: f64, t24682: f64, t27607: f64, t1209: f64, t85821: f64, t15743: f64, t7345: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t95270, t95273, t95276, t95303, t95304, t95320) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2103(t15437, t24728, t24732, t4965, t7344, t1184, t24682, t27607, t1209, t85821, t15743, t7345);
    (t95270, t95273, t95276, t95303, t95304, t95320)
}
