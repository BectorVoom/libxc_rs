//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 605/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk605<F: Float>(t10420: F, t5182: F, t10414: F, t5196: F, t10358: F, t10362: F, t10368: F, t10373: F, t10377: F, t10379: F, t10384: F, t10387: F, t10392: F, t10397: F, t10402: F, t10406: F, t10410: F, t10415: F, t10417: F) -> (F, F, F) {
    let t10421 = t5182 * t10420;
    let t10423 = t10414 * t5196;
    let t10425 = -0.49745833333333333332e-2 * t10358 + 0.33163888888888888887e-2 * t10362 + 0.99491666666666666664e-2 * t10368 - 0.8290972222222222222e-2 * t10373 + 0.49745833333333333332e-2 * t10377 + 0.66327777777777777776e-2 * t10379 + 0.99491666666666666664e-2 * t10384 + 0.49745833333333333332e-2 * t10387 - 0.49745833333333333332e-2 * t10392 + 0.33163888888888888887e-2 * t10397 - 0.99491666666666666664e-2 * t10402 + 0.82909722222222222219e-2 * t10406 + 0.44218518518518518516e-2 * t10410 - 0.99491666666666666664e-2 * t10415 - 0.66327777777777777776e-2 * t10417 + 0.99491666666666666664e-2 * t10421 + 0.66327777777777777776e-2 * t10423;
    (t10421, t10423, t10425)
}
