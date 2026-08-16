//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1961/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961(t12461: f64, t7939: f64, t29376: f64, t532: f64, t193: f64, t200: f64, t7844: f64, t1877: f64, t2057: f64, t24191: f64, t25015: f64, t25021: f64, t2522: f64, t25366: f64, t25392: f64, t26563: f64, t26744: f64, t28252: f64, t7110: f64, t7114: f64, t92319: f64, t97956: f64, t97990: f64, t98004: f64, t98008: f64, t98059: f64, t98079: f64, t98094: f64, t99049: f64, t99056: f64) -> (f64, f64, f64, f64) {
    let t101138 = t7939 * t12461;
    let t101150 = t532 * t29376;
    let t101196 = t193 * t200 * t7844;
    let t101209 = -3.0_f64 * t92319 * t25021 - 3.0_f64 * t24191 * t98079 - 3.0_f64 * t24191 * t99049 - 3.0_f64 * t24191 * t98008 + 3.0_f64 * t26563 * t99056 - t1877 * t7114 * t97990 - 6.0_f64 * t26563 * t98059 - 3.0_f64 * t92319 * t25366 - 3.0_f64 * t26563 * t97956 + 6.0_f64 * t101196 * t25015 - t1877 * t26744 * t25392 + 3.0_f64 * t24191 * t98004 + 3.0_f64 * t2522 * t7110 * t28252 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t98094;
    (t101138, t101150, t101196, t101209)
}
