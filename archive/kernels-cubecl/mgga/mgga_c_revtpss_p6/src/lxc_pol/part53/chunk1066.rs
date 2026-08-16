//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1066/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1066<F: Float>(t33942: F, t33973: F, t532: F, t1450: F, t2014: F, t2042: F, t7944: F, t2040: F, t7950: F, t7953: F, t1916: F, t8611: F) -> (F, F, F, F, F, F, F, F) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t33996 = t7944 * t2042;
    let t33998 = t2040 * t7950;
    let t34000 = t2040 * t7953;
    let t34003 = F::cast_from(6.0_f64) * t1916 * t8611;
    (t33974, t33975, t33976, t33977, t33996, t33998, t34000, t34003)
}
