//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1352/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1352<F: Float>(t113: F, t7197: F, t2155: F, t8077: F, t2634: F, t481: F, t22790: F, t6086: F, t6407: F, t7949: F, t2133: F, t2574: F, t6848: F, t20771: F, t20782: F, t2122: F, t2124: F, t25191: F, t2551: F, t25555: F, t25557: F, t25560: F, t25564: F, t25567: F, t25571: F) -> (F, F) {
    let t25573 = t7197 * t113;
    let t25575 = t2155 * t8077 * t25573;
    let t25577 = t2634 * t481;
    let t25579 = t22790 * t6086 * t25577;
    let t25581 = t6407 * t7949;
    let t25582 = 0.17563392970889009434e0 * t25581;
    let t25584 = t2133 * t6848 * t2574;
    let t25585 = 0.12713391885412927226e1 * t25584;
    let t25590 = -0.16463622957338778996e-1 * t20771 + 0.20803732176130244552e1 * t25555 + 0.10401866088065122276e1 * t25557 - 0.48787202696913915094e-3 * t20782 + 0.1047928639570397803e0 * t25560 - 0.87816964854445047168e-1 * t25564 - 0.32927245914677557992e-1 * t25567 + 0.58544643236296698111e-1 * t25571 - 0.29272321618148349056e-1 * t25575 - 0.20958572791407956061e0 * t25579 - t25582 + t25585 + 0.16463622957338778996e0 * t2122 * t2124 * t25191 * t2551;
    (t25573, t25590)
}
