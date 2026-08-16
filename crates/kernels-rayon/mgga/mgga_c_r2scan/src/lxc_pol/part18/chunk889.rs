//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 889/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk889(t2609: f64, t7601: f64, t3187: f64, t6518: f64, t2662: f64, t2670: f64, t3183: f64, t5108: f64, t568: f64, t6106: f64, t6493: f64, t6505: f64, t6509: f64, t6513: f64, t7250: f64, t8265: f64, t8268: f64, t8272: f64, t944: f64, t9469: f64, t9478: f64, t9482: f64, t9485: f64, t9488: f64) -> f64 {
    let t9490 = t7601 * t2609;
    let t9498 = t6518 * t3187;
    let t9500 = -0.13002332610081402845e0_f64 * t9469 * t568 + 0.5200933044032561138e0_f64 * t6493 * t3183 - 0.12713391885412927226e1_f64 * t6505 - 0.42377972951376424087e0_f64 * t6509 + 0.34672886960217074253e0_f64 * t9478 - 0.42683466926433871472e0_f64 * t6513 - 0.2600466522016280569e0_f64 * t5108 * t9482 - 0.10401866088065122276e1_f64 * t6106 * t9485 - 0.48787202696913915093e-2_f64 * t9488 - 0.11643651550782197811e-1_f64 * t9490 - t8265 + 0.1358426014257923078e0_f64 * t8268 + 0.4075278042773769234e0_f64 * t8272 - 0.2600466522016280569e0_f64 * t7250 * t944 - 0.2600466522016280569e0_f64 * t2670 * t2662 - 0.38415120233790484326e0_f64 * t9498;
    t9500
}
