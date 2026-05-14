//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 821/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk821<F: Float>(t29287: F, t655: F, t2464: F, t8814: F, t10873: F, t1310: F, t17220: F, t17222: F, t1773: F, t23338: F, t23413: F, t23416: F, t23769: F, t2466: F, t29055: F, t29061: F, t5013: F, t664: F, t7208: F, t7219: F, t8816: F, t8822: F, sigma2: F) -> (F,) {
    let t29288 = t29287 * sigma2;
    let t29289 = t29288 * t655;
    let t29296 = t8814 * t2464;
    let t29297 = t10873 * t29296;
    let t29298 = t1310 * t29297;
    let t29301 = 0.95950873152945691803e-1 * t17220 - 0.35981577432354634426e-1 * t17222 + 0.53972366148531951639e-1 * t23413 - 0.53972366148531951639e-1 * t5013 * t29055 - 0.5397236614853195164e-1 * t1773 * t29061 - 0.16191709844559585492e0 * t7208 * t8822 - 0.86355785837651122625e0 * t7219 * t8816 + 0.43177892918825561313e0 * t7219 * t8822 - 0.35981577432354634425e-1 * t23416 + 0.5397236614853195164e-1 * t29289 * t664 + 0.86355785837651122625e0 * t23338 * t2466 - 0.15831894070236039148e1 * t23769 * t2466 - 0.32383419689119170984e0 * t1773 * t29298;
    (t29301,)
}
