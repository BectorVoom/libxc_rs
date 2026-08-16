//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1193/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1193(t6071: f64, t886: f64, t2770: f64, t10673: f64, t14675: f64, t14690: f64, t14703: f64, t14705: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64) -> (f64, f64) {
    let t18323 = t6071 * t886;
    let t18324 = t2770 * t18323;
    let t18330 = t14675 - t14690 + t14703 + t14705 + t10673 - 0.11337795902333997111e-1_f64 * t14712 + t14715 + 0.27104001498285508386e-3_f64 * t14716 - t14722 + t14726 - t14730 - t14734;
    (t18324, t18330)
}
