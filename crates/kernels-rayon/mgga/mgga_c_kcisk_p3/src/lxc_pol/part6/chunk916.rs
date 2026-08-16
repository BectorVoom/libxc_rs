//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 916/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk916(t29287: f64, t655: f64, t2464: f64, t8814: f64, t10873: f64, t1310: f64, t17220: f64, t17222: f64, t1773: f64, t23338: f64, t23413: f64, t23416: f64, t23769: f64, t2466: f64, t29055: f64, t29061: f64, t5013: f64, t664: f64, t7208: f64, t7219: f64, t8816: f64, t8822: f64, sigma2: f64) -> f64 {
    let t29288 = t29287 * sigma2;
    let t29289 = t29288 * t655;
    let t29296 = t8814 * t2464;
    let t29297 = t10873 * t29296;
    let t29298 = t1310 * t29297;
    let t29301 = 0.95950873152945691803e-1_f64 * t17220 - 0.35981577432354634426e-1_f64 * t17222 + 0.53972366148531951639e-1_f64 * t23413 - 0.53972366148531951639e-1_f64 * t5013 * t29055 - 0.5397236614853195164e-1_f64 * t1773 * t29061 - 0.16191709844559585492e0_f64 * t7208 * t8822 - 0.86355785837651122625e0_f64 * t7219 * t8816 + 0.43177892918825561313e0_f64 * t7219 * t8822 - 0.35981577432354634425e-1_f64 * t23416 + 0.5397236614853195164e-1_f64 * t29289 * t664 + 0.86355785837651122625e0_f64 * t23338 * t2466 - 0.15831894070236039148e1_f64 * t23769 * t2466 - 0.32383419689119170984e0_f64 * t1773 * t29298;
    t29301
}
