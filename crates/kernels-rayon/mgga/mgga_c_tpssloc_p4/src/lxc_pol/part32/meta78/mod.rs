//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk516;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta78(t1539: f64, t882: f64, t123: f64, t881: f64, t291: f64, t880: f64, t894: f64, t901: f64, t908: f64, t136: f64, t899: f64, t907: f64, t913: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1540, t1541, t1543, t1545, t1547) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk516(t1539, t882, t123, t881, t291, t880);
        let (t1548, t1551, t1553, t1554, t1556, t1557) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk517(t1547, t894, t901, t1539, t908, t136, t1541, t899, t907, t913);
    (t1540, t1541, t1543, t1545, t1547, t1548, t1551, t1553, t1554, t1556, t1557)
}
