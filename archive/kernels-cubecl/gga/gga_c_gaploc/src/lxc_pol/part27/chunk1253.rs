//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1253/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1253<F: Float>(t33182: F, t10939: F, t5694: F, t2617: F, t2963: F, t7803: F, t10834: F, t22883: F, t32893: F, t7427: F, t7573: F, t11026: F, t5782: F) -> (F, F, F, F, F, F) {
    let t33183 = F::cast_from(0.76685851907841499352e0_f64) * t33182;
    let t33187 = F::cast_from(0.92686455430723328401e-1_f64) * t10939 * t5694;
    let t33193 = t7803 * t2963 * t2617;
    let t33194 = F::cast_from(0.38342925953920749676e0_f64) * t33193;
    let t33195 = t22883 * t10834;
    let t33196 = F::cast_from(0.29792074959875355558e-1_f64) * t33195;
    let t33205 = F::cast_from(0.62115540045351614476e2_f64) * t7427 * t7573 * t32893;
    let t33210 = F::cast_from(0.13803453343411469884e2_f64) * t5782 * t11026;
    (t33183, t33187, t33194, t33196, t33205, t33210)
}
