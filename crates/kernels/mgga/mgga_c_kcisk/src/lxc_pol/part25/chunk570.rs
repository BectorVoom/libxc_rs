//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 570/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk570<F: Float>(t1857: F, t970: F, t1856: F, t4648: F, t1835: F, t4640: F, t1836: F, t960: F, t706: F, t1843: F, t965: F, t1842: F, t1659: F, t167: F, t4597: F, t158: F, t165: F, t173: F, t1850: F, t3290: F, t3293: F, t5089: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5142 = t970 * t1857;
    let t5144 = t1856 * t4648;
    let t5147 = t1835 * t4640;
    let t5150 = t960 * t1836;
    let t5152 = t1835 * t4648;
    let t5155 = t706 * t4640;
    let t5158 = t965 * t1843;
    let t5160 = t1842 * t4648;
    let t5163 = t1659 * t4640;
    let t5168 = t167 * t4597;
    let t5171 = -0.5179538907796306876e-4 * t1850 * t3293 - 0.23526125e-4 * t5142 + 0.50413125e-5 * t173 * t5144 - 0.672175e-5 * t173 * t5147 + 0.9368e-2 * t5150 - 0.3513e-2 * t158 * t5152 + 0.1171e-2 * t158 * t5155 - 0.26416666666666666666e-2 * t5158 + 0.7925e-3 * t165 * t5160 - 0.52833333333333333333e-3 * t165 * t5163 - 0.23911438650126355246e-1 * t5089 * t3290 + 0.15538616723388920628e-3 * t5168 * t3290;
    (t5142, t5144, t5147, t5150, t5152, t5155, t5158, t5160, t5163, t5168, t5171)
}
