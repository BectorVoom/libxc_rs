//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 972/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk972<F: Float>(t17240: F, t5015: F, t2454: F, t3934: F, t649: F, t1849: F, t2464: F, t3290: F, t16017: F, t7242: F, t2364: F, t5032: F, t11179: F, t10800: F, t10810: F, t10828: F, t10856: F, t10863: F, t17218: F, t17220: F, t17222: F, t17226: F, t17230: F, t17235: F, t5013: F, t5017: F, t5022: F, t7208: F, t7235: F, t7239: F, t7243: F, t7258: F) -> (F,) {
    let t17241 = t5015 * t17240;
    let t17248 = t649 * t2454 * t3934;
    let t17251 = t2464 * t1849;
    let t17252 = t17251 * t3290;
    let t17253 = t5015 * t17252;
    let t17256 = t7242 * t16017;
    let t17259 = t2364 * t5032;
    let t17260 = t11179 * t17259;
    let t17264 = -0.35981577432354634426e-1 * t7208 * t5022 - 0.11993859144118211475e-1 * t10800 - 0.35981577432354634426e-1 * t10856 * t7239 - 0.71963154864709268852e-1 * t10856 * t7243 - 0.11993859144118211475e-1 * t10810 - 0.95950873152945691806e-1 * t17218 + 0.31983624384315230602e-1 * t17220 - 0.11993859144118211475e-1 * t17222 - 0.23987718288236422951e-1 * t5013 * t17226 - 0.17990788716177317213e-1 * t5013 * t17230 - 0.35981577432354634426e-1 * t5013 * t17235 - 0.35981577432354634426e-1 * t10856 * t7258 - 0.17990788716177317213e-1 * t5013 * t17241 + 0.11993859144118211475e-1 * t10828 + 0.47975436576472845902e-1 * t10856 * t7235 + 0.95950873152945691806e-1 * t17248 * t5017 + 0.35981577432354634426e-1 * t5013 * t17253 + 0.10794473229706390328e0 * t5013 * t17256 + 0.35981577432354634426e-1 * t5013 * t17260 - 0.35981577432354634426e-1 * t10863;
    (t17264,)
}
