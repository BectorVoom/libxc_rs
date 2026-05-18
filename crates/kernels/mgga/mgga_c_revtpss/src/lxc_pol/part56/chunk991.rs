//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 991/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk991<F: Float>(t31808: F, t31829: F, t31833: F, t31850: F, t33695: F, t33699: F, t33704: F, t33708: F, t33712: F, t33717: F, t33719: F, t33723: F, t8481: F, t8649: F) -> F {
    let t33725 = t31808 + F::new(0.57119737665102352616e0) * t33695 * t8481 - F::new(0.17135921299530705785e1) * t8649 * t33699 - F::new(0.11423947533020470523e1) * t8649 * t33704 + F::new(0.11423947533020470523e1) * t8649 * t33708 + t31829 - t31833 - F::new(0.1859366460452550541e-3) * t33712 + F::new(0.3718732920905101082e-3) * t33717 + F::new(0.3718732920905101082e-3) * t33719 + t31850 + F::new(0.7437465841810202164e-3) * t33723;
    t33725
}
