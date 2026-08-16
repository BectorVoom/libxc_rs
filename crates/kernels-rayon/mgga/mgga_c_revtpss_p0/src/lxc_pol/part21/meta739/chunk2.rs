//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2594/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2594(t10146: f64, t10147: f64, t13743: f64, t1424: f64, t14299: f64, t1903: f64, t4071: f64, t4076: f64, t4078: f64, t46353: f64, t46356: f64, t46359: f64, t47764: f64, t47772: f64, t47777: f64, t47781: f64, t47785: f64, t47786: f64, t47791: f64, t47793: f64, t47794: f64, t5715: f64, t9651: f64) -> f64 {
    let t47798 = -0.11853808529283920877e2_f64 * t4071 * t13743 + 0.19637199382202157274e-3_f64 * t47764 + 0.13170898365871023197e1_f64 * t1424 * t4076 * t1903 * t10146 + 0.39512695097613069591e1_f64 * t14299 * t4078 + 0.11044544084478153697e-3_f64 * t47772 - 0.43902994552903410657e-1_f64 * t46353 - 0.65854491829355115987e0_f64 * t5715 * t10147 + 0.11708928647259339623e0_f64 * t47777 - 0.19637199382202157274e-3_f64 * t47781 + t47785 - 0.2601984143835408805e-2_f64 * t47786 - 0.39029762157531132075e-2_f64 * t46356 + 0.58544643236296698112e-1_f64 * t47791 + t46359 - 0.11853808529283920877e2_f64 * t47793 * t47794 * t9651;
    t47798
}
