//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1852/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1852(t15123: f64, t15125: f64, t15128: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18951: f64, t18977: f64, t18980: f64, t18982: f64, t18985: f64, t18988: f64, t18990: f64, t18993: f64, t18995: f64, t19019: f64) -> f64 {
    let t19021 = -0.33547222222222222222e0_f64 * t18906 + 0.12077e1_f64 * t18911 - 0.40256666666666666666e0_f64 * t18915 + 0.16504875e0_f64 * t18951 - 0.18396666666666666667e0_f64 * t15123 - 0.40256666666666666668e0_f64 * t15125 + t15128 - 0.181155e1_f64 * t18928 + 0.12077e1_f64 * t18932 - 0.20128333333333333333e0_f64 * t18939 + t18977 + 0.19419375e1_f64 * t18980 - 0.258925e1_f64 * t18982 - 0.1294625e1_f64 * t18985 - 0.412621875e-1_f64 * t18988 + 0.16504875e0_f64 * t18990 + 0.82524375e-1_f64 * t18993 + 0.258925e1_f64 * t18995 + 0.67094444444444444443e-1_f64 * t18919 - 0.20128333333333333333e0_f64 * t18924 + 0.10064166666666666667e0_f64 * t18934 + t19019;
    t19021
}
