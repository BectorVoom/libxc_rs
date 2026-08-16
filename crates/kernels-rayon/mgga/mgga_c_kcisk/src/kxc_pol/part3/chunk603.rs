//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 603/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk603(t1659: f64, t4640: f64, t167: f64, t4597: f64, t158: f64, t165: f64, t173: f64, t1850: f64, t3290: f64, t3293: f64, t5089: f64, t5142: f64, t5144: f64, t5147: f64, t5150: f64, t5152: f64, t5155: f64, t5158: f64, t5160: f64) -> (f64, f64, f64) {
    let t5163 = t1659 * t4640;
    let t5168 = t167 * t4597;
    let t5171 = -0.5179538907796306876e-4_f64 * t1850 * t3293 - 0.23526125e-4_f64 * t5142 + 0.50413125e-5_f64 * t173 * t5144 - 0.672175e-5_f64 * t173 * t5147 + 0.9368e-2_f64 * t5150 - 0.3513e-2_f64 * t158 * t5152 + 0.1171e-2_f64 * t158 * t5155 - 0.26416666666666666666e-2_f64 * t5158 + 0.7925e-3_f64 * t165 * t5160 - 0.52833333333333333333e-3_f64 * t165 * t5163 - 0.23911438650126355246e-1_f64 * t5089 * t3290 + 0.15538616723388920628e-3_f64 * t5168 * t3290;
    (t5163, t5168, t5171)
}
