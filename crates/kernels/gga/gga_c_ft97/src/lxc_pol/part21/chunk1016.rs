//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1016/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1016<F: Float>(t20522: F, t342: F, t630: F, t1017: F, t15567: F, t15752: F, t15756: F, t15763: F, t15768: F, t16633: F, t16640: F, t16659: F, t16661: F, t16664: F, t16919: F, t16932: F, t2258: F, t2984: F, t2993: F, t343: F, t41305: F, t41328: F, t41332: F, t61123: F, t64663: F, t64668: F, t64677: F, t64681: F, t72: F, t8633: F) -> (F,) {
    let t78700 = t342 * t630 * t20522;
    let t78725 = t64663 / 27.0 - t64668 - t342 * t343 * t72 * t16919 / 4.0 - t78700 / 12.0 - t15567 * t16640 * t15752 / 2.0 + 2.0 / 3.0 * t61123 * t16640 * t15756 + t15567 * t16640 * t15768 / 6.0 + t64677 / 9.0 - t64681 + t15567 * t2258 * t1017 * t2993 / 3.0 - 2.0 / 9.0 * t15567 * t8633 * t1017 * t2984 + t41332 / 54.0 - t15567 * t16633 * t15763 / 9.0 + t41305 / 18.0 - t41328 + t16664 + t16659 + t16932 + t16661;
    (t78725,)
}
