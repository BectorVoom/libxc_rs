//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 783/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk783(t2222: f64, t3517: f64, t2188: f64, t3598: f64, t2226: f64, t11313: f64, t2218: f64, t1354: f64, t2083: f64, t2079: f64, t3676: f64, t2089: f64, t2877: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19163 = t3517 * t2222;
    let t19182 = t3598 * t2188;
    let t19235 = t3517 * t2226;
    let t19404 = t11313 * t2218;
    let t19434 = t1354 * t2083;
    let t19476 = t2079 * t3676;
    let t19543 = t2877 * t2089;
    (t19163, t19182, t19235, t19404, t19434, t19476, t19543)
}
