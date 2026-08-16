//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1249/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1249(t33182: f64, t10939: f64, t5694: f64, t2617: f64, t2963: f64, t7803: f64, t10834: f64, t22883: f64, t32893: f64, t7427: f64, t7573: f64, t11026: f64, t5782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33183 = 0.76685851907841499352e0_f64 * t33182;
    let t33187 = 0.92686455430723328401e-1_f64 * t10939 * t5694;
    let t33193 = t7803 * t2963 * t2617;
    let t33194 = 0.38342925953920749676e0_f64 * t33193;
    let t33195 = t22883 * t10834;
    let t33196 = 0.29792074959875355558e-1_f64 * t33195;
    let t33205 = 0.62115540045351614476e2_f64 * t7427 * t7573 * t32893;
    let t33210 = 0.13803453343411469884e2_f64 * t5782 * t11026;
    (t33183, t33187, t33194, t33196, t33205, t33210)
}
