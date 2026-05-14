//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1346/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1346<F: Float>(t2139: F, t2294: F, t7945: F, t1593: F, t20494: F, t20497: F, t20505: F, t20507: F, t20516: F, t20520: F, t20997: F, t22780: F, t2562: F, t360: F, t6132: F, t6149: F, t6152: F, t6364: F, t6370: F, t6583: F, t7433: F, t7450: F, t7512: F, t7539: F, t7991: F, t8001: F, t8110: F) -> (F,) {
    let t25442 = t2139 * t2294 * t7945;
    let t25446 = 0.54878743191129263322e-2 * t20494 + 0.7801399566048841707e0 * t6152 * t7450 - 0.26004665220162805689e0 * t6583 * t360 * t7433 * t8110 + 0.2600466522016280569e0 * t20997 * t360 * t2562 * t6364 - 0.15602799132097683414e1 * t7512 * t360 * t8001 * t1593 + 0.13002332610081402845e0 * t6149 * t7991 - 0.26004665220162805689e0 * t6132 * t360 * t2562 * t6370 + 0.69345773920434148506e0 * t20497 + 0.20803732176130244552e1 * t20505 + 0.76830240467580968652e0 * t20507 - 0.26004665220162805689e0 * t22780 * t7539 - 0.10401866088065122276e1 * t25442 + 0.52396431978519890151e-1 * t20516 + 0.52396431978519890151e-1 * t20520;
    (t25446,)
}
