//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1113/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1113<F: Float>(t33182: F, t10939: F, t5694: F, t2617: F, t2963: F, t7803: F, t10834: F, t22883: F, t32893: F, t7427: F, t7573: F, t11026: F, t5782: F, t11030: F, t2365: F, t24741: F, t6111: F) -> (F, F, F, F, F, F, F, F) {
    let t33183 = 0.76685851907841499352e0 * t33182;
    let t33187 = 0.92686455430723328401e-1 * t10939 * t5694;
    let t33193 = t7803 * t2963 * t2617;
    let t33194 = 0.38342925953920749676e0 * t33193;
    let t33195 = t22883 * t10834;
    let t33196 = 0.29792074959875355558e-1 * t33195;
    let t33205 = 0.62115540045351614476e2 * t7427 * t7573 * t32893;
    let t33210 = 0.13803453343411469884e2 * t5782 * t11026;
    let t33212 = 0.13803453343411469884e2 * t5782 * t11030;
    let t33214 = t6111 * t2365 * t24741;
    (t33183, t33187, t33194, t33196, t33205, t33210, t33212, t33214)
}
