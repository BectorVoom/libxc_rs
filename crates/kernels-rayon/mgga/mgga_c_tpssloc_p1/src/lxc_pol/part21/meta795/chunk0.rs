//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2756/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2756(t46308: f64, t46310: f64, t16616: f64, t2371: f64, t40794: f64, t40804: f64, t40806: f64, t46317: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58055 = 0.11696447245269292414e1_f64 * t46308;
    let t58056 = 0.23392894490538584828e1_f64 * t46310;
    let t58057 = t16616 * t2371;
    let t58058 = 0.11696447245269292414e1_f64 * t58057;
    let t58059 = 0.32530743900905219526e-1_f64 * t40794;
    let t58060 = 0.65061487801810439052e-1_f64 * t40804;
    let t58061 = 0.96319466275353142156e0_f64 * t40806;
    let t58062 = 8.0_f64 * t46317;
    let t58063 = -t58055 - t58056 + t58058 + t40790 + t40793 + t58059 + t40797 + t40799 + t40801 - t40803 - t58060 + t58061 + t58062;
    (t58055, t58056, t58058, t58059, t58060, t58061, t58062, t58063)
}
