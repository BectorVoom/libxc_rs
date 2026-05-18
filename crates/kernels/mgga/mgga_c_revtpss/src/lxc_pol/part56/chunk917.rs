//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 917/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk917<F: Float>(t31846: F, t839: F, t846: F, t8486: F, t241: F, t853: F, t125: F, t246: F, t775: F, t31808: F, t31809: F, t31814: F, t31820: F, t31824: F, t31829: F, t31833: F, t31835: F, t31842: F, t8481: F, t8649: F) -> (F, F, F, F, F) {
    let t31847 = t31846 * t839;
    let t31849 = t8486 * t846;
    let t31850 = F::new(0.86770434821119025247e-3) * t31849;
    let t31851 = t241 * t853;
    let t31853 = t246 * t125 * t775;
    let t31854 = t31851 * t31853;
    let t31855 = t8486 * t31854;
    let t31857 = t31808 + F::new(0.57119737665102352616e0) * t31809 * t8481 - F::new(0.17135921299530705785e1) * t8649 * t31814 - F::new(0.11423947533020470523e1) * t8649 * t31820 + F::new(0.11423947533020470523e1) * t8649 * t31824 + t31829 - t31833 - F::new(0.1859366460452550541e-3) * t31835 + F::new(0.3718732920905101082e-3) * t31842 + F::new(0.3718732920905101082e-3) * t31847 + t31850 + F::new(0.7437465841810202164e-3) * t31855;
    (t31850, t31851, t31853, t31854, t31857)
}
