//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 917/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk917<F: Float>(t2169: F, t2219: F, t1584: F, t1592: F, t1638: F, t2187: F, t2198: F, t562: F, t568: F, t6437: F, t6440: F, t6446: F, t6449: F, t6452: F, t6455: F, t6459: F, t6463: F, t6465: F, t6468: F, t6472: F, t6478: F, t6483: F, t6487: F, t6490: F, t6493: F) -> (F, F) {
    let t6496 = t2169 * t2219;
    let t6498 = 0.39006997830244208535e0 * t1592 * t6437 + 0.69345773920434148506e0 * t6440 - 0.13002332610081402845e0 * t1584 * t1638 + 0.34672886960217074253e0 * t6446 - 0.15602799132097683414e1 * t6449 * t6452 - 0.34672886960217074253e0 * t6455 + 0.10401866088065122276e1 * t6459 - 0.86743646395112941037e-3 * t6463 + 0.26004665220162805689e0 * t6465 * t2187 + 0.69345773920434148506e0 * t6468 + 0.34672886960217074253e0 * t6472 - 0.19043987679069580388e-1 * t6478 - 0.57131963037208741166e-1 * t6483 - 0.13002332610081402845e0 * t6487 * t562 - 0.39006997830244208535e0 * t6490 * t568 + 0.15602799132097683414e1 * t6493 * t2198 + 0.20803732176130244552e1 * t6496;
    (t6496, t6498)
}
