//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1012/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1012<F: Float>(t121028: F, t3140: F, t9656: F, t1385: F, t1404: F, t32276: F, t32278: F, t3985: F, t8591: F, t240: F, t843: F, t31752: F, t32197: F, t8477: F, t8705: F, t1419: F, t31805: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121029 = 0.3718732920905101082e-4 * t121028;
    let t121034 = t3140 * t9656;
    let t121035 = t121034 * t1385;
    let t121043 = t32276 * t1404 * t32278;
    let t121044 = 0.34708173928447610098e-2 * t121043;
    let t121045 = t8591 * t3985;
    let t121056 = t1385 * t843 * t240;
    let t121057 = t31752 * t121056;
    let t121058 = t121057 * t32197;
    let t121059 = 0.263521689745817692e-2 * t121058;
    let t121076 = t8477 * t8705 * t9656;
    let t121099 = t31805 * t1419;
    (t121029, t121034, t121035, t121044, t121045, t121057, t121059, t121076, t121099)
}
