//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1074/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1074<F: Float>(t1526: F, t3323: F, t7705: F, t2252: F, t342: F, t4645: F, t16654: F, t630: F, t11003: F, t11050: F, t11059: F, t12561: F, t12597: F, t13239: F, t142: F, t15567: F, t16633: F, t16640: F, t2248: F, t343: F, t358: F, t41305: F, t41328: F, t41332: F, t41335: F, t41338: F, t41341: F, t41344: F, t41358: F, t422: F, t61123: F, t72: F) -> (F,) {
    let t64668 = t1526 * t7705 * t3323 / 18.0;
    let t64677 = t342 * t2252 * t4645;
    let t64681 = t342 * t630 * t16654 / 6.0;
    let t64701 = -t64668 - t15567 * t16640 * t11050 / 2.0 + 2.0 / 3.0 * t15567 * t16633 * t11059 + 2.0 * t13239 + t64677 / 18.0 - t64681 + t41332 / 27.0 - t41335 / 18.0 - t41338 / 36.0 - t41341 / 27.0 - t342 * t343 * t72 * t12561 / 4.0 + 2.0 / 3.0 * t61123 * t16640 * t11003 - t41344 / 12.0 + t12597 + t41305 / 9.0 - t41328 + t41358 / 18.0 - t1526 * t2248 * t422 * t142 * t358 / 6.0;
    (t64701,)
}
