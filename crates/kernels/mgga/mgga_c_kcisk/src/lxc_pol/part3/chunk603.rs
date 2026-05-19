//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 603/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk603<F: Float>(t1659: F, t4640: F, t167: F, t4597: F, t158: F, t165: F, t173: F, t1850: F, t3290: F, t3293: F, t5089: F, t5142: F, t5144: F, t5147: F, t5150: F, t5152: F, t5155: F, t5158: F, t5160: F) -> (F, F, F) {
    let t5163 = t1659 * t4640;
    let t5168 = t167 * t4597;
    let t5171 = -F::cast_from(0.5179538907796306876e-4_f64) * t1850 * t3293 - F::new(0.23526125e-4) * t5142 + F::new(0.50413125e-5) * t173 * t5144 - F::new(0.672175e-5) * t173 * t5147 + F::new(0.9368e-2) * t5150 - F::new(0.3513e-2) * t158 * t5152 + F::new(0.1171e-2) * t158 * t5155 - F::cast_from(0.26416666666666666666e-2_f64) * t5158 + F::new(0.7925e-3) * t165 * t5160 - F::cast_from(0.52833333333333333333e-3_f64) * t165 * t5163 - F::cast_from(0.23911438650126355246e-1_f64) * t5089 * t3290 + F::cast_from(0.15538616723388920628e-3_f64) * t5168 * t3290;
    (t5163, t5168, t5171)
}
