//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 972/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk972<F: Float>(t1955: F, t7997: F, t1579: F, t8651: F, t31812: F, t1568: F, t3140: F, t8477: F, t1959: F, t32434: F, t32460: F, t32473: F, t32476: F, t32480: F, t32483: F, t33675: F, t33712: F, t33719: F, t7770: F, t7775: F, t8649: F, t8652: F) -> (F, F, F, F, F, F) {
    let t34063 = t1955 * t7997;
    let t34068 = t8651 * t1579;
    let t34069 = t31812 * t34068;
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34078 = 0.17347256376410398924e1 * t32434 * t7770 + t32460 - 0.3718732920905101082e-3 * t33712 - t32473 + t32476 - 0.8673628188205199462e0 * t34063 * t1959 + 0.7437465841810202164e-3 * t33719 - 0.56468933516960933999e-3 * t33675 + t32480 - t32483 - 0.17135921299530705785e1 * t8649 * t34069 + 0.8673628188205199462e0 * t32434 * t7775 + 0.57119737665102352616e0 * t34075 * t8652;
    (t34063, t34068, t34069, t34074, t34075, t34078)
}
