//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1028/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1028(t1415: f64, t8684: f64, t2488: f64, t8678: f64, t2487: f64, t3781: f64, t849: f64, t2496: f64, t3773: f64, t2504: f64, t3789: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t11080: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11082 = t8684 * t1415;
    let t11083 = t11082 * t2488;
    let t11085 = t8678 * t1415;
    let t11086 = t11085 * t2488;
    let t11088 = t2487 * t3781;
    let t11089 = t11088 * t849;
    let t11091 = t3773 * t2496;
    let t11093 = t2504 * t3781;
    let t11094 = t11093 * t849;
    let t11096 = t3789 * t2496;
    let t11098 = -0.19931111111111111111e0_f64 * t11024 - 0.17938e1_f64 * t11028 + 0.11958666666666666667e1_f64 * t11033 + 0.59793333333333333334e0_f64 * t11037 + 0.3071625e0_f64 * t11080 + 0.142419375e1_f64 * t11083 - 0.76790625e-1_f64 * t11086 - 0.1898925e1_f64 * t11089 - 0.9494625e0_f64 * t11091 + 0.3071625e0_f64 * t11094 + 0.15358125e0_f64 * t11096;
    (t11083, t11086, t11089, t11091, t11094, t11096, t11098)
}
