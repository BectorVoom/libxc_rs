//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 830/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk830(t13959: f64, t8177: f64, t1458: f64, t8185: f64, t1413: f64, t8231: f64, t25350: f64, t492: f64, t4265: f64, t8220: f64, t8224: f64, t8212: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27037 = t13959 * t8177;
    let t27047 = t8185 * t1458;
    let t27180 = t8231 * t1413;
    let t27181 = t27180 * sigma0;
    let t27204 = t25350 * t492;
    let t27270 = t4265 * t8220;
    let t27308 = t4265 * t8224;
    let t27319 = t4265 * t8212;
    (t27037, t27047, t27181, t27204, t27270, t27308, t27319)
}
