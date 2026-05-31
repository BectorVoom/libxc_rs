//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1150/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1150<F: Float>(t1171: F, t4574: F, t3778: F, t510: F, t1543: F, t2903: F, t11575: F, t11578: F, t11583: F, t11586: F, t11589: F, t11594: F, t11597: F, t1163: F, t3697: F, t3714: F, t3786: F, t4541: F, t9598: F, t9612: F, t9624: F, t9625: F, t9737: F, t9742: F, t9765: F, t9769: F, t9773: F) -> (F, F, F, F) {
    let t11605 = t4574 * t1171;
    let t11608 = t510 * t3778;
    let t11611 = t2903 * t1543;
    let t11614 = t3786 * t4541 - F::cast_from(360.0_f64) * t9769 * t11575 + F::cast_from(504.0_f64) * t9773 * t11578 + F::cast_from(24.0_f64) * t9765 * t11578 + F::cast_from(400.0_f64) * t9612 * t11583 - F::cast_from(400.0_f64) * t11586 * t3714 + F::cast_from(1400.0_f64) / F::cast_from(3.0_f64) * t11589 * t3714 + F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t9598 * t11583 - F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t11594 * t3714 + F::cast_from(12.0_f64) * t9742 * t11597 - F::cast_from(180.0_f64) * t9624 * t9625 * t3697 + F::cast_from(252.0_f64) * t9737 * t11597 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11605 * t1163 + F::cast_from(200.0_f64) / F::cast_from(3.0_f64) * t11608 * t3714 - F::cast_from(1000.0_f64) / F::cast_from(3.0_f64) * t11611 * t3714;
    (t11605, t11608, t11611, t11614)
}
