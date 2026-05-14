//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1290/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1290<F: Float>(t2469: F, t32935: F, t5038: F, t7261: F, t116145: F, t1799: F, t5187: F, t34093: F, t5195: F, t10494: F, t34209: F, t1785: F, t7274: F, t116361: F, t116368: F, t116370: F, t116372: F, t116375: F, t116378: F, t116380: F, t32938: F, t34073: F, t34154: F, t9664: F) -> (F, F, F, F, F, F) {
    let t116384 = t7261 * t32935 * t2469 * t5038;
    let t116388 = t1799 * t116145 * t5187;
    let t116391 = t1799 * t34093 * t5195;
    let t116393 = t10494 * t34209;
    let t116394 = 0.3684876543209876543e-2 * t116393;
    let t116397 = t7261 * t32935 * t7274 * t1785;
    let t116400 = 0.11054629629629629629e-2 * t116361 - 0.20833333333333333334e-1 * t34073 * t32938 - 0.8041666666666666667e-2 * t34154 * t32938 - t116368 - t116370 - 0.16203703703703703704e-1 * t116372 - 0.33163888888888888888e-2 * t116375 - 0.16581944444444444444e-1 * t116378 - 0.22109259259259259258e-2 * t116380 - 0.10416666666666666667e-1 * t9664 * t116384 + 0.88437037037037037034e-2 * t116388 - 0.5895802469135802469e-2 * t116391 + t116394 - 0.20833333333333333334e-1 * t9664 * t116397;
    (t116384, t116388, t116391, t116393, t116397, t116400)
}
