//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1046/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1046<F: Float>(t2670: F, t3192: F, t3224: F, t7313: F, t8268: F, t8272: F, t8275: F, t8277: F, t940: F, t944: F, t9447: F, t9453: F, t9458: F, t9464: F, t9469: F, t9478: F, t9488: F, t9490: F, t9498: F, t9551: F, t9554: F) -> (F,) {
    let t10386 = -0.39006997830244208535e0 * t2670 * t3224 + 0.26004665220162805689e0 * t7313 * t3192 - 0.13002332610081402845e0 * t9464 * t940 - 0.39006997830244208535e0 * t9469 * t944 - 0.17465477326173296717e-1 * t9447 + 0.34672886960217074253e0 * t9453 + 0.20803732176130244552e1 * t9458 + 0.10401866088065122276e1 * t9478 - 0.14636160809074174528e-1 * t9488 - 0.34930954652346593433e-1 * t9490 + 0.2037639021386884617e0 * t8268 + 0.6112917064160653851e0 * t8272 - 0.11524536070137145298e1 * t9498 - 0.2037639021386884617e0 * t8275 - 0.98781737744032673979e-1 * t8277 - 0.76830240467580968651e0 * t9551 + 0.87816964854445047168e-1 * t9554;
    (t10386,)
}
