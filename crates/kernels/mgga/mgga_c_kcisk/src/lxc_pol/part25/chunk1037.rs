//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1037/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1037<F: Float>(t18426: F, t2023: F, t7261: F, t15909: F, t5497: F, t7246: F, t4998: F, t7628: F, t2013: F, t2020: F, t7233: F, t5492: F, t6758: F, t12248: F, t12249: F, t12251: F, t12263: F, t12266: F, t12269: F, t18326: F, t18403: F, t18406: F, t18408: F, t18410: F, t18414: F, t18423: F, t5471: F, t5494: F, t5499: F, t7591: F, t7629: F, t7634: F, t788: F) -> (F, F) {
    let t18427 = t18426 * t2023;
    let t18428 = t7261 * t18427;
    let t18434 = t5497 * t15909;
    let t18435 = t7246 * t18434;
    let t18442 = t4998 * t7628;
    let t18443 = t2013 * t18442;
    let t18445 = t7233 * t2020;
    let t18446 = t6758 * t5492;
    let t18447 = t18445 * t18446;
    let t18450 = 0.17990788716177317213e-1 * t2013 * t18403 - 0.59969295720591057378e-2 * t18406 + 0.15991812192157615301e-1 * t18408 - 0.71963154864709268853e-1 * t18410 * t788 + 0.53972366148531951639e-1 * t2013 * t18414 + t12248 + 0.89953943580886586067e-2 * t12249 - 0.11993859144118211476e-1 * t12251 + 0.10794473229706390328e0 * t5471 * t7634 + t18423 + 0.47975436576472845902e-1 * t7591 * t5499 + 0.10794473229706390328e0 * t2013 * t18428 + 0.11993859144118211476e-1 * t12263 - 0.89953943580886586067e-2 * t12266 + 0.17990788716177317213e-1 * t12269 + 0.71963154864709268852e-1 * t2013 * t18435 + 0.47975436576472845902e-1 * t7591 * t5494 - 0.17990788716177317213e-1 * t5471 * t7629 - 0.59969295720591057378e-2 * t18443 - 0.23987718288236422952e-1 * t18326 * t18447;
    (t18446, t18450)
}
