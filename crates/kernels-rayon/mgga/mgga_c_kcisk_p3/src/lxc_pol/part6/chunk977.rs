//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 977/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk977(t30212: f64, t3796: f64, t3482: f64, t2075: f64, t8251: f64, t3484: f64, t5886: f64, t7907: f64, t1411: f64, t2236: f64, t25308: f64, t2231: f64, t8072: f64) -> (f64, f64, f64, f64, f64) {
    let t30213 = t3796 * t30212;
    let t30214 = t3482 * t30213;
    let t30216 = t8251 * t2075;
    let t30217 = t3484 * t30216;
    let t30218 = t3482 * t30217;
    let t30220 = t5886 * t7907;
    let t30221 = t1411 * t30220;
    let t30223 = t25308 * t2236;
    let t30224 = t1411 * t30223;
    let t30226 = t8072 * t2231;
    (t30214, t30218, t30221, t30224, t30226)
}
