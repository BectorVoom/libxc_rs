//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 650/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk650<F: Float>(t10412: F, t1336: F, t140: F, t5188: F, t10409: F, t10370: F, t6666: F, t5184: F, t5182: F, t5196: F, t10358: F, t10362: F, t10368: F, t10373: F, t10377: F, t10379: F, t10384: F, t10387: F, t10392: F, t10397: F, t10402: F, t10406: F, t10410: F) -> (F, F, F, F, F) {
    let t10414 = t140 * t1336 * t10412;
    let t10415 = t10414 * t5188;
    let t10417 = t10409 * t5188;
    let t10419 = t6666 * t10370;
    let t10420 = t5184 * t10419;
    let t10421 = t5182 * t10420;
    let t10423 = t10414 * t5196;
    let t10425 = -F::cast_from(0.49745833333333333332e-2_f64) * t10358 + F::cast_from(0.33163888888888888887e-2_f64) * t10362 + F::cast_from(0.99491666666666666664e-2_f64) * t10368 - F::cast_from(0.8290972222222222222e-2_f64) * t10373 + F::cast_from(0.49745833333333333332e-2_f64) * t10377 + F::cast_from(0.66327777777777777776e-2_f64) * t10379 + F::cast_from(0.99491666666666666664e-2_f64) * t10384 + F::cast_from(0.49745833333333333332e-2_f64) * t10387 - F::cast_from(0.49745833333333333332e-2_f64) * t10392 + F::cast_from(0.33163888888888888887e-2_f64) * t10397 - F::cast_from(0.99491666666666666664e-2_f64) * t10402 + F::cast_from(0.82909722222222222219e-2_f64) * t10406 + F::cast_from(0.44218518518518518516e-2_f64) * t10410 - F::cast_from(0.99491666666666666664e-2_f64) * t10415 - F::cast_from(0.66327777777777777776e-2_f64) * t10417 + F::cast_from(0.99491666666666666664e-2_f64) * t10421 + F::cast_from(0.66327777777777777776e-2_f64) * t10423;
    (t10415, t10417, t10421, t10423, t10425)
}
