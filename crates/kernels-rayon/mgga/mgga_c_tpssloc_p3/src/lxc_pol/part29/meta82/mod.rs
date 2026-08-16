//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk538;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta82(t1088: f64, t1653: f64, t123: f64, t1087: f64, t423: f64, t1086: f64, t1100: f64, t1107: f64, t1113: f64, t136: f64, t1105: f64, t1112: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1654, t1655, t1657, t1659, t1661) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk538(t1088, t1653, t123, t1087, t423, t1086);
        let (t1662, t1665, t1667, t1668, t1670, t1671) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk539(t1100, t1661, t1107, t1113, t1653, t136, t1105, t1112, t1655, t1118);
    (t1654, t1655, t1657, t1659, t1661, t1662, t1665, t1667, t1668, t1670, t1671)
}
