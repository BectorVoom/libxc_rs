//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2594/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2594<F: Float>(t10146: F, t10147: F, t13743: F, t1424: F, t14299: F, t1903: F, t4071: F, t4076: F, t4078: F, t46353: F, t46356: F, t46359: F, t47764: F, t47772: F, t47777: F, t47781: F, t47785: F, t47786: F, t47791: F, t47793: F, t47794: F, t5715: F, t9651: F) -> F {
    let t47798 = -F::cast_from(0.11853808529283920877e2_f64) * t4071 * t13743 + F::cast_from(0.19637199382202157274e-3_f64) * t47764 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t4076 * t1903 * t10146 + F::cast_from(0.39512695097613069591e1_f64) * t14299 * t4078 + F::cast_from(0.11044544084478153697e-3_f64) * t47772 - F::cast_from(0.43902994552903410657e-1_f64) * t46353 - F::cast_from(0.65854491829355115987e0_f64) * t5715 * t10147 + F::cast_from(0.11708928647259339623e0_f64) * t47777 - F::cast_from(0.19637199382202157274e-3_f64) * t47781 + t47785 - F::cast_from(0.2601984143835408805e-2_f64) * t47786 - F::cast_from(0.39029762157531132075e-2_f64) * t46356 + F::cast_from(0.58544643236296698112e-1_f64) * t47791 + t46359 - F::cast_from(0.11853808529283920877e2_f64) * t47793 * t47794 * t9651;
    t47798
}
