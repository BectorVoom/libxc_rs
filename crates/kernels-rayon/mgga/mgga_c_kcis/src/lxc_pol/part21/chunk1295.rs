//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1295/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1295(t4580: f64, t95547: f64, t95830: f64, t13156: f64, t2842: f64, t7718: f64, t26748: f64, t26808: f64, t27773: f64, t27832: f64, t27950: f64, t2850: f64, t2894: f64, t2911: f64, t44657: f64, t7703: f64, t93592: f64, t95816: f64, t95817: f64, t95820: f64, t95827: f64, t95828: f64) -> (f64, f64, f64) {
    let t95832 = t95830 * t4580 * t95547;
    let t95844 = t2842 * t7718 * t13156;
    let t95846 = -t95816 - 0.73697530864197530861e-3_f64 * t95817 - 0.33163888888888888888e-2_f64 * t95820 + 0.41703125000000000001e-2_f64 * t7703 * t44657 * t27773 * t2911 + t95827 - 0.55273148148148148147e-2_f64 * t95828 - 0.92673611111111111112e-3_f64 * t93592 * t95832 - 0.46336805555555555556e-3_f64 * t7703 * t2894 * t27773 * t2850 + 0.46336805555555555556e-3_f64 * t27832 * t26808 - 0.61782407407407407408e-3_f64 * t26748 * t27950 - 0.16581944444444444444e-1_f64 * t95844;
    (t95832, t95844, t95846)
}
