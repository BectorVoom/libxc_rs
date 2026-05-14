//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1028/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1028<F: Float>(t122295: F, t32275: F, t94390: F, t28911: F, t8584: F, t25875: F, t25901: F, t32268: F, t2470: F, t32706: F, t32705: F, t120996: F, t122282: F, t7286: F, t786: F, t2453: F, t25946: F, t32715: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t122309 = 0.50779446784275991476e-2 * t94390 * t32275 * t122295;
    let t122310 = t8584 * t28911;
    let t122311 = t25875 * t122310;
    let t122312 = t122311 * t25901;
    let t122314 = t32268 * t122310;
    let t122315 = t122314 * t25901;
    let t122317 = t32706 * t2470;
    let t122319 = 0.19039912555034117539e-1 * t32705 * t122317;
    let t122321 = 0.7052700942260554372e-3 * t120996;
    let t122327 = t786 * t122282 * t7286;
    let t122331 = 0.3427046870806409921e-2 * t2453 * t32715 * t25946;
    (t122309, t122311, t122312, t122314, t122315, t122317, t122319, t122321, t122327, t122331)
}
