//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 727/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk727<F: Float>(t44626: F, t1063: F, t11271: F, t6750: F, t2268: F, t3565: F, t6763: F, t13310: F, t484: F, t36247: F, t894: F, t426: F, t44294: F, t535: F, t13273: F, t11254: F, t2343: F, t6519: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44627 = 0.11856252764865062333e-2 * t44626;
    let t44630 = 0.85365019907028448797e-1 * t1063 * t11271 * t6750;
    let t44633 = 0.42682509953514224398e0 * t2268 * t3565 * t6763;
    let t44634 = t484 * t13310;
    let t44635 = 0.15808337019820083111e-2 * t44634;
    let t44638 = 0.28455006635676149599e-1 * t1063 * t894 * t36247;
    let t44642 = 0.28455006635676149599e-1 * t2268 * t535 * t44294 * t426;
    let t44643 = t484 * t13273;
    let t44644 = 0.31616674039640166221e-2 * t44643;
    let t44658 = 0.19918504644973304719e0 * t2268 * t11271 * t6763;
    let t44662 = 0.56910013271352299198e-1 * t1063 * t2343 * t11254 * t6519;
    (t44627, t44630, t44633, t44635, t44638, t44642, t44644, t44658, t44662)
}
