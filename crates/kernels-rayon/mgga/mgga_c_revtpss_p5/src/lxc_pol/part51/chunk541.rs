//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 541/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk541(t2847: f64, t2848: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t291: f64, t1596: f64, t914: f64, t936: f64, t1610: f64, t2869: f64) -> (f64, f64, f64) {
    let t4587 = t2847 + 0.5936111111111111111e-2_f64 * t2848 + 0.5936111111111111111e-2_f64 * t4571 - 0.11872222222222222222e-1_f64 * t4576 + 0.35616666666666666666e-1_f64 * t4581 - 0.17808333333333333333e-1_f64 * t4585;
    let t4589 = 0.621814e-1_f64 * t4587 * t291;
    let t4590 = t1596 * t914;
    let t4592 = 1.0_f64 * t4590 * t936;
    let t4594 = 1.0_f64 * t2869 * t1610;
    (t4589, t4592, t4594)
}
