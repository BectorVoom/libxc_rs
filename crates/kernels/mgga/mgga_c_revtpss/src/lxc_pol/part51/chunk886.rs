//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 886/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk886<F: Float>(t31838: F, t33716: F, t1561: F, t31846: F, t246: F, t4450: F, t31851: F, t8486: F, t31808: F, t31829: F, t31833: F, t31850: F, t33695: F, t33699: F, t33704: F, t33708: F, t33712: F, t8481: F, t8649: F) -> (F, F, F) {
    let t33717 = t31838 * t33716;
    let t33719 = t31846 * t1561;
    let t33721 = t246 * t4450;
    let t33722 = t31851 * t33721;
    let t33723 = t8486 * t33722;
    let t33725 = t31808 + 0.57119737665102352616e0 * t33695 * t8481 - 0.17135921299530705785e1 * t8649 * t33699 - 0.11423947533020470523e1 * t8649 * t33704 + 0.11423947533020470523e1 * t8649 * t33708 + t31829 - t31833 - 0.1859366460452550541e-3 * t33712 + 0.3718732920905101082e-3 * t33717 + 0.3718732920905101082e-3 * t33719 + t31850 + 0.7437465841810202164e-3 * t33723;
    (t33721, t33722, t33725)
}
