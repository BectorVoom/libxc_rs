//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1073/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1073<F: Float>(t1045: F, t1985: F, t16641: F, t61128: F, t16634: F, t142: F, t39430: F, t1526: F, t16644: F, t45751: F, t3338: F, t7705: F, t38308: F, t4641: F, t10998: F, t11008: F, t11013: F, t11034: F, t11280: F, t12283: F, t12288: F, t12298: F, t12302: F, t12313: F, t12330: F, t13234: F, t1527: F, t15567: F, t16633: F, t16640: F, t2258: F, t2984: F, t2993: F, t3088: F, t558: F, t61123: F, t8633: F) -> (F, F) {
    let t64242 = t1985 * t1045;
    let t64621 = t61128 * t16641 / 9.0;
    let t64623 = 2.0 / 27.0 * t61128 * t16634;
    let t64631 = t39430 * t142;
    let t64642 = t1526 * t45751 * t16644;
    let t64655 = t1526 * t7705 * t3338 / 18.0;
    let t64663 = t1526 * t38308 * t4641;
    let t64665 = t15567 * t2258 * t558 * t2993 / 3.0 + t15567 * t16640 * t10998 / 6.0 + t64621 - t64623 - 2.0 / 9.0 * t15567 * t8633 * t558 * t2984 - t15567 * t16633 * t11034 / 9.0 - 7.0 / 27.0 * t15567 * t64631 * t11008 - 4.0 / 9.0 * t61123 * t16633 * t11013 + t13234 + t1526 * t1527 * t12313 / 6.0 + 7.0 / 18.0 * t64642 - t1526 * t1527 * t12298 / 12.0 - t1526 * t1527 * t12283 / 12.0 - t1526 * t3088 * t12288 / 9.0 - t64655 - t1526 * t1527 * t12330 / 6.0 - t1526 * t11280 * t12302 / 3.0 + t64663 / 54.0;
    (t64242, t64665)
}
