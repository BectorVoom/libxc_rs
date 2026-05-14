//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 752/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk752<F: Float>(t45320: F, t7427: F, t7573: F, t2615: F, t326: F, t45305: F, t11603: F, t2464: F, t2465: F, t13638: F, t7416: F, t11627: F, t2684: F, t825: F, t10930: F, t10931: F) -> (F, F, F, F, F, F, F) {
    let t45323 = 0.12423108009070322895e3 * t7427 * t7573 * t45320;
    let t45326 = 0.46011511144704899612e1 * t2615 * t326 * t45305;
    let t45329 = t7427 * t2464 * t2465 * t11603;
    let t45331 = t7416 * t13638;
    let t45335 = t2684 * t2464 * t2465 * t11627;
    let t45343 = 0.18404604457881959845e2 * t825 * t326 * t45320;
    let t45349 = 0.55213813373645879534e2 * t10930 * t10931 * t45320;
    (t45323, t45326, t45329, t45331, t45335, t45343, t45349)
}
