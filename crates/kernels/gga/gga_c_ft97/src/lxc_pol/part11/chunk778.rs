//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 778/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk778<F: Float>(t1751: F, t35: F, t401: F, t428: F, t1300: F, t15811: F, t1594: F, t1603: F, t1618: F, t1624: F, t1631: F, t1681: F, t1701: F, t1702: F, t1712: F, t2035: F, t2037: F, t22603: F, t22605: F, t3065: F, t372: F, t37519: F, t37525: F, t37526: F, t37529: F, t37537: F, t37541: F, t37545: F, t37551: F, t37552: F, t37554: F, t37558: F, t37570: F, t37574: F, t37578: F, t37582: F, t37591: F, t534: F, t7837: F, t7838: F, t7839: F, t7843: F, t7845: F, t7867: F, t7877: F, t7883: F, t7889: F, t7914: F, t8009: F, t8015: F, t8139: F, t8161: F) -> (F,) {
    let t37594 = t35 * t1751;
    let t37595 = t37594 * t401;
    let t37599 = t37594 * t428;
    let t37607 = -0.5509824679191440163e-4 * t37525 * t37526 + 0.16540877980489188955e-3 * t8015 * t37529 + 0.82704389902445944776e-3 * t7845 * t8161 * t7839 + 0.55098246791914401631e-4 * t8009 * t37529 + 0.33081755960978377911e-3 * t37537 * t37519 + 0.10475889387643153005e-1 * t7838 * t1618 * t37541 - 0.52379446938215765024e-2 * t22603 * t37545 * t1681 * t22605 - 0.16540877980489188955e-2 * t37551 * t37552 * t37554 - 0.25803162535905570824e-4 * t1603 * t534 * t37558 - 0.84321219226603029515e-3 * t7867 * t2035 * t2037 * t1751 - 0.23709522591370051951e-1 * t1300 * t1701 * t1702 * t8139 + 0.279058811357253504e-1 * t1603 * t1631 * t37570 + 0.1116235245429014016e-1 * t1624 * t7914 * t37574 - 0.1674352868143521024e-1 * t372 * t7914 * t37578 - 0.11619434043764639964e-2 * t1624 * t1594 * t37582 + 0.33081755960978377911e-2 * t7837 * t7843 * t3065 * t37591 + 0.279058811357253504e0 * t7877 * t3065 * t37595 - 0.279058811357253504e0 * t15811 * t3065 * t37599 - 0.45048092923603098704e0 * t7889 * t1701 * t7883 * t1712;
    (t37607,)
}
