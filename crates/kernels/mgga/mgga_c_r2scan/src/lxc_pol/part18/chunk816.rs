//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 816/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk816<F: Float>(t2662: F, t2670: F, t3183: F, t5108: F, t568: F, t6106: F, t6493: F, t6505: F, t6509: F, t6513: F, t7250: F, t8265: F, t8268: F, t8272: F, t944: F, t9469: F, t9478: F, t9482: F, t9485: F, t9488: F, t9490: F, t9498: F) -> (F,) {
    let t9500 = -0.13002332610081402845e0 * t9469 * t568 + 0.5200933044032561138e0 * t6493 * t3183 - 0.12713391885412927226e1 * t6505 - 0.42377972951376424087e0 * t6509 + 0.34672886960217074253e0 * t9478 - 0.42683466926433871472e0 * t6513 - 0.2600466522016280569e0 * t5108 * t9482 - 0.10401866088065122276e1 * t6106 * t9485 - 0.48787202696913915093e-2 * t9488 - 0.11643651550782197811e-1 * t9490 - t8265 + 0.1358426014257923078e0 * t8268 + 0.4075278042773769234e0 * t8272 - 0.2600466522016280569e0 * t7250 * t944 - 0.2600466522016280569e0 * t2670 * t2662 - 0.38415120233790484326e0 * t9498;
    (t9500,)
}
