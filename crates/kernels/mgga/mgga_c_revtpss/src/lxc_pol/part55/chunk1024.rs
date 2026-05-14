//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1024/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1024<F: Float>(t121901: F, t25413: F, t32429: F, t686: F, t72: F, t32469: F, t32440: F, t2061: F, t786: F, t25410: F, t119989: F, t1955: F, t2769: F, t32433: F, t10073: F, t25403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t121902 = t121901 * t25413;
    let t121913 = t32429 * t72 * t686;
    let t121914 = t32469 * t121913;
    let t121920 = t32440 * t72 * t686;
    let t121921 = t32469 * t121920;
    let t121940 = t786 * t2061;
    let t121941 = t121940 * t25410;
    let t121942 = t121941 * t25413;
    let t121946 = 0.7052700942260554372e-3 * t119989;
    let t121975 = t1955 * t32433 * t2769;
    let t121980 = 0.4818682326780666368e-3 * t10073 * t32433 * t25403;
    (t121902, t121913, t121914, t121920, t121921, t121940, t121941, t121942, t121946, t121975, t121980)
}
