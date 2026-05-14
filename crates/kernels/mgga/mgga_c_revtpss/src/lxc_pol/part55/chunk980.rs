//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 980/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk980<F: Float>(t2030: F, t32681: F, t32682: F, t32683: F, t32709: F, t32712: F, t32718: F, t32719: F, t33923: F, t33931: F, t33960: F, t33967: F, t34204: F, t34212: F, t7930: F, t8702: F) -> (F,) {
    let t34216 = -0.8673628188205199462e0 * t34204 * t2030 + t32681 + t32682 - t32683 - 0.3718732920905101082e-3 * t33960 - 0.56468933516960933999e-3 * t33931 - 0.56468933516960933999e-3 * t33923 - 0.8673628188205199462e0 * t8702 * t7930 - 0.11423947533020470523e1 * t32719 * t34212 - t32709 + t32712 + 0.7437465841810202164e-3 * t33967 - t32718;
    (t34216,)
}
