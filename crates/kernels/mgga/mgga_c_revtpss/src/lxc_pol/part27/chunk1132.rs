//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1132/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1132<F: Float>(t1936: F, t49851: F, t10416: F, t7002: F, t49693: F, t13435: F, t2322: F, t25832: F, t60551: F, t13440: F, t5523: F, t112: F, t843: F, t239: F, t655: F, t665: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94958 = 6.0 * t49851 * t1936;
    let t94960 = 6.0 * t10416 * t7002;
    let t94962 = 6.0 * t49693 * t1936;
    let t94964 = 12.0 * t13435 * t7002;
    let t94966 = 6.0 * t2322 * t25832;
    let t94968 = 2.0 * t60551 * t1936;
    let t94970 = 6.0 * t13440 * t7002;
    let t94972 = 6.0 * t5523 * t25832;
    let t94973 = t843 * t112;
    let t94974 = 154.0 / 27.0 * t94973;
    let t94975 = t239 * t655;
    let t94976 = t94975 * t665;
    (t94958, t94960, t94962, t94964, t94966, t94968, t94970, t94972, t94974, t94976)
}
