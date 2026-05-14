//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1375/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1375<F: Float>(t33162: F, t34444: F, t34465: F, t9733: F, t33276: F, t9991: F, t34462: F, t9736: F, t10000: F, t10005: F, t10009: F, t113082: F, t116859: F, t116888: F, t2807: F, t33173: F, t33258: F, t33263: F, t33283: F, t34435: F, t34469: F, t34473: F, t9732: F, t9990: F) -> (F,) {
    let t118206 = 0.13402777777777777778e-2 * t34444 * t33162;
    let t118210 = 0.34722222222222222222e-2 * t9733 * t34465;
    let t118212 = t9991 * t33276;
    let t118223 = t34462 * t9736;
    let t118229 = -0.10416666666666666667e-1 * t10000 * t33263 + t118206 + 0.40208333333333333334e-2 * t33258 * t34469 + t118210 - 0.11607361111111111111e-2 * t116859 + 0.11574074074074074074e-2 * t118212 - 0.10416666666666666667e-1 * t34473 * t9732 * t2807 - 0.52083333333333333333e-2 * t9990 * t33283 * t2807 + 0.27777777777777777778e-1 * t10005 * t33263 - 0.25794135802469135802e-3 * t116888 + 0.92592592592592592594e-2 * t118223 - 0.17361111111111111111e-2 * t113082 * t10009 - 0.17361111111111111111e-2 * t34435 * t33173;
    (t118229,)
}
