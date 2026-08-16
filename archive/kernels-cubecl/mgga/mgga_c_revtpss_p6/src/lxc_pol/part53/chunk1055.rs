//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1055/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1055<F: Float>(t31808: F, t31829: F, t31833: F, t31850: F, t33695: F, t33699: F, t33704: F, t33708: F, t33712: F, t33717: F, t33719: F, t33723: F, t8481: F, t8649: F) -> F {
    let t33725 = t31808 + F::cast_from(0.57119737665102352616e0_f64) * t33695 * t8481 - F::cast_from(0.17135921299530705785e1_f64) * t8649 * t33699 - F::cast_from(0.11423947533020470523e1_f64) * t8649 * t33704 + F::cast_from(0.11423947533020470523e1_f64) * t8649 * t33708 + t31829 - t31833 - F::cast_from(0.1859366460452550541e-3_f64) * t33712 + F::cast_from(0.3718732920905101082e-3_f64) * t33717 + F::cast_from(0.3718732920905101082e-3_f64) * t33719 + t31850 + F::cast_from(0.7437465841810202164e-3_f64) * t33723;
    t33725
}
