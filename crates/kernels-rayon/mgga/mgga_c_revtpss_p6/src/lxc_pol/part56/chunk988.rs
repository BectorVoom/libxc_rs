//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 988/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk988(t31748: f64, t31751: f64, t31764: f64, t31783: f64, t31786: f64, t31787: f64, t31794: f64, t31803: f64, t33675: f64, t33679: f64, t33683: f64, t33688: f64, t33691: f64, t7779: f64, t8472: f64) -> f64 {
    let t33694 = -t31748 + t31751 - 0.28234466758480466999e-3_f64 * t33675 - t31764 - 0.112937867033921868e-2_f64 * t33679 - 0.28234466758480466999e-3_f64 * t33683 + t31783 - t31786 - 0.17347256376410398924e1_f64 * t31787 * t7779 + 0.17347256376410398924e1_f64 * t8472 * t33688 + 0.8673628188205199462e0_f64 * t31794 * t33691 - t31803;
    t33694
}
