//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1337/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1337(t19046: f64, t7338: f64, t6169: f64, t7344: f64, t18375: f64, t7339: f64, t27599: f64, t4997: f64, t18329: f64, t7310: f64, t18324: f64, t18371: f64, t24741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104015 = t19046 * t7338;
    let t104018 = t6169 * t7344;
    let t104048 = t7339 * t18375;
    let t104050 = t27599 * t4997;
    let t104085 = t7310 * t18329;
    let t104088 = t7310 * t18324;
    let t104094 = t24741 * t18371;
    (t104015, t104018, t104048, t104050, t104085, t104088, t104094)
}
