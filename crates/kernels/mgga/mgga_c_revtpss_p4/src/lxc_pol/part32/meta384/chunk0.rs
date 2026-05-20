//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1344/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1344<F: Float>(t16708: F, t16710: F, t16712: F, t1256: F, t5258: F, t5262: F, t1804: F, t3655: F, t1786: F, t1260: F, t12987: F, t15687: F, t3623: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17319 = F::cast_from(0.37037037037037037037e-2_f64) * t16708;
    let t17320 = F::cast_from(0.11111111111111111111e-1_f64) * t16710;
    let t17321 = F::cast_from(0.55555555555555555556e-2_f64) * t16712;
    let t17337 = F::cast_from(0.15244095330869239812e-2_f64) * t5258 * t1256;
    let t17339 = F::cast_from(0.28582678745379824648e-3_f64) * t5262 * t1256;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    (t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350)
}
