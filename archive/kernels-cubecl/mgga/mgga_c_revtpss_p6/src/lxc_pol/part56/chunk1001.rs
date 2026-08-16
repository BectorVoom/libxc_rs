//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1001/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1001<F: Float>(t33942: F, t33973: F, t532: F, t1450: F, t2014: F, t1916: F, t8611: F, t1518: F, t8453: F, t572: F, t7330: F, t7741: F) -> (F, F, F, F, F, F, F, F) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t34003 = F::cast_from(6.0_f64) * t1916 * t8611;
    let t34004 = t1518 * t8453;
    let t34006 = F::cast_from(6.0_f64) * t572 * t34004;
    let t34007 = t7330 * t7741;
    (t33974, t33975, t33976, t33977, t34003, t34004, t34006, t34007)
}
