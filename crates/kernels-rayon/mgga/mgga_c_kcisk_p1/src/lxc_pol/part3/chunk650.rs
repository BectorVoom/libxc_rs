//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 650/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk650(t10412: f64, t1336: f64, t140: f64, t5188: f64, t10409: f64, t10370: f64, t6666: f64, t5184: f64, t5182: f64, t5196: f64, t10358: f64, t10362: f64, t10368: f64, t10373: f64, t10377: f64, t10379: f64, t10384: f64, t10387: f64, t10392: f64, t10397: f64, t10402: f64, t10406: f64, t10410: f64) -> (f64, f64, f64, f64, f64) {
    let t10414 = t140 * t1336 * t10412;
    let t10415 = t10414 * t5188;
    let t10417 = t10409 * t5188;
    let t10419 = t6666 * t10370;
    let t10420 = t5184 * t10419;
    let t10421 = t5182 * t10420;
    let t10423 = t10414 * t5196;
    let t10425 = -0.49745833333333333332e-2_f64 * t10358 + 0.33163888888888888887e-2_f64 * t10362 + 0.99491666666666666664e-2_f64 * t10368 - 0.8290972222222222222e-2_f64 * t10373 + 0.49745833333333333332e-2_f64 * t10377 + 0.66327777777777777776e-2_f64 * t10379 + 0.99491666666666666664e-2_f64 * t10384 + 0.49745833333333333332e-2_f64 * t10387 - 0.49745833333333333332e-2_f64 * t10392 + 0.33163888888888888887e-2_f64 * t10397 - 0.99491666666666666664e-2_f64 * t10402 + 0.82909722222222222219e-2_f64 * t10406 + 0.44218518518518518516e-2_f64 * t10410 - 0.99491666666666666664e-2_f64 * t10415 - 0.66327777777777777776e-2_f64 * t10417 + 0.99491666666666666664e-2_f64 * t10421 + 0.66327777777777777776e-2_f64 * t10423;
    (t10415, t10417, t10421, t10423, t10425)
}
