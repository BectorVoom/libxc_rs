//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1857/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1857(t27492: f64, t3317: f64, t1671: f64, t25512: f64, t25522: f64, t25526: f64, t25535: f64, t25538: f64, t25580: f64, t4825: f64, t4869: f64, t4875: f64, t4887: f64, t4902: f64, t4907: f64, t4912: f64, t7111: f64, t7122: f64) -> (f64, f64) {
    let t27498 = t3317 * t27492;
    let t27518 = -0.42874018118069736972e-3_f64 * t27498 * t4902 - t25535 / 108.0_f64 - t25538 + t7111 * t4887 / 288.0_f64 - 0.42874018118069736972e-3_f64 * t25580 * t4907 - 0.42874018118069736972e-3_f64 * t25580 * t4912 - 0.28582678745379824648e-3_f64 * t25522 * t4825 + 0.42874018118069736972e-3_f64 * t25512 * t1671 + 0.42874018118069736972e-3_f64 * t7122 * t4869 - 0.28582678745379824648e-3_f64 * t25522 * t4875 - 0.22866142996303859718e-2_f64 * t25526 * t1671;
    (t27498, t27518)
}
