//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2756;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta795(t46308: f64, t46310: f64, t16616: f64, t2371: f64, t40794: f64, t40804: f64, t40806: f64, t46317: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t4119: f64, t868: f64, t12652: f64, t12939: f64, t4195: f64, t1462: f64, t47172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58055, t58056, t58058, t58059, t58060, t58061, t58062, t58063) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2756(t46308, t46310, t16616, t2371, t40794, t40804, t40806, t46317, t40790, t40793, t40797, t40799, t40801, t40803);
        let (t58071, t58080, t58085, t58090) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2757(t4119, t868, t12652, t12939, t4195, t1462, t47172);
    (t58055, t58056, t58058, t58059, t58060, t58061, t58062, t58063, t58071, t58080, t58085, t58090)
}
