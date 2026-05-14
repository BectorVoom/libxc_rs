//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1365/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1365<F: Float>(t62760: F, t79: F, t2803: F, t33176: F, t9999: F, t33183: F, t34484: F, t1586: F, t2805: F, t60805: F, t112807: F, t113111: F, t2804: F, t33180: F, t33188: F, t33263: F, t34444: F, t34462: F, t34469: F, t34477: F, t34573: F, t34580: F, t9721: F, t9728: F, t9748: F, t9991: F, t9995: F) -> (F, F) {
    let t117951 = t62760 * t79;
    let t117952 = t117951 * t2803;
    let t117961 = t33176 * t9999;
    let t117967 = 0.13402777777777777778e-2 * t33183 * t34484;
    let t117969 = t1586 * t2805 * t60805;
    let t117980 = 0.10416666666666666667e-1 * t34477 * t9728 + 0.20104166666666666667e-2 * t34444 * t33188 - 0.116403125e-2 * t117952 * t33180 + 0.20104166666666666667e-2 * t113111 * t9995 + 0.10416666666666666667e-1 * t9721 * t34469 - 0.53611111111111111112e-2 * t34573 * t33188 - 0.116403125e-2 * t117961 * t33180 + 0.40208333333333333334e-2 * t112807 * t9995 + t117967 + 0.52083333333333333333e-2 * t2804 * t117969 - 0.27777777777777777778e-1 * t34462 * t9748 - 0.10416666666666666667e-1 * t9991 * t33263 - 0.27777777777777777778e-1 * t34580 * t9748 - 0.27777777777777777778e-1 * t34580 * t9728;
    (t117969, t117980)
}
