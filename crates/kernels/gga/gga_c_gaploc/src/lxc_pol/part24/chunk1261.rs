//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1261/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1261<F: Float>(t10388: F, t10489: F, t10492: F, t1445: F, t1555: F, t1580: F, t1617: F, t1641: F, t1646: F, t31382: F, t31386: F, t31394: F, t31502: F, t31800: F, t31829: F, t3371: F, t35172: F, t35174: F, t35178: F, t35183: F, t35185: F, t35188: F, t567: F, t574: F, t597: F) -> (F,) {
    let t35189 = -0.71500979903700853338e0 * t1555 * t3371 * t1646 + 0.46011511144704899612e1 * t567 * t1445 * t31502 + 0.46011511144704899612e1 * t1617 * t10388 - 0.92023022289409799224e1 * t1641 * t10489 + 0.23005755572352449806e2 * t1580 * t10492 - 0.92023022289409799224e1 * t574 * t1445 * t31800 + 0.23005755572352449806e2 * t597 * t1445 * t31829 + t35172 - t35174 - t35178 - t31382 + t31386 + t31394 + t35183 - t35185 + t35188;
    (t35189,)
}
