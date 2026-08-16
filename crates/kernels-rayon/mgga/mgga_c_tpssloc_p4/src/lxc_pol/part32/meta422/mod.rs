//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1627;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta422(t11883: f64, t1215: f64, t6252: f64, t1751: f64, t5011: f64, t1246: f64, t6238: f64, t19145: f64, t3612: f64, t1734: f64, t5052: f64, t1235: f64, t6218: f64, t19120: f64, t493: f64, t1243: f64, t19045: f64, t1755: f64, t11881: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1758: f64, t18572: f64, t3604: f64, t3610: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5073: f64, t5076: f64, t5086: f64, t6168: f64, t6257: f64, t6265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19165, t19166, t19169, t19170, t19173, t19174, t19176, t19179, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1627(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
        let (t19201, t19203, t19207) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1628(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
    (t19165, t19169, t19173, t19179, t19189, t19201, t19203, t19207)
}
