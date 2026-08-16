//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1034/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1034(t23682: f64, t23685: f64, t2414: f64, t216: f64, t2417: f64, t2568: f64, t212: f64, t2263: f64, t362: f64, t508: f64, t896: f64, t297: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24287 = 0.31003950617283950618e1_f64 * t23682;
    let t24288 = 0.13388493827160493828e1_f64 * t23685;
    let t24300 = t2414 * t2414;
    let t24302 = t216 / t24300;
    let t24304 = t2417 * t2417;
    let t24305 = 1.0_f64 / t24304;
    let t24321 = 0.96141975308641975307e-1_f64 * t23682;
    let t24356 = t2568 * t2568;
    let t24357 = 1.0_f64 / t24356;
    let t24391 = 1.0_f64 / t212 / t2263;
    let t24392 = t24391 * t362;
    let t24407 = t508 * t896;
    let t24442 = t935 * t297;
    (t24287, t24288, t24302, t24305, t24321, t24357, t24391, t24392, t24407, t24442)
}
