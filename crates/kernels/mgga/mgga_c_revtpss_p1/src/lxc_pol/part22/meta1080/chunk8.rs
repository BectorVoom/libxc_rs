//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3890/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3890<F: Float>(t14104: F, t47856: F, t13729: F, t2782: F, t556: F, t5774: F, t1424: F, t213: F, t225: F, t4077: F, t47904: F, t47907: F, t47913: F, t47918: F, t47920: F, t47926: F, t47929: F, t47932: F, t47936: F, t47938: F, t47942: F, t47944: F, t561: F, t6918: F, t73705: F, t73707: F, t73712: F, t74724: F, t9657: F) -> F {
    let t74733 = t47856 * t14104;
    let t74744 = t2782 * t556 * t13729 * t5774;
    let t74749 = -F::cast_from(0.60712963356159538786e-1_f64) * t47904 + F::cast_from(0.10975748638225852664e-1_f64) * t73705 + F::cast_from(0.14634331517634470219e-1_f64) * t73707 + F::cast_from(0.2601984143835408805e-2_f64) * t47907 - F::cast_from(0.11565819519348392139e-2_f64) * t73712 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t74724 * t225 * t561 - F::cast_from(0.11708928647259339623e0_f64) * t47913 - F::cast_from(0.19514881078765566038e-1_f64) * t47918 - F::cast_from(0.92526556154787137113e-2_f64) * t47920 - F::cast_from(0.43902994552903410656e-1_f64) * t47926 - F::cast_from(0.23131639038696784277e-2_f64) * t74733 + F::cast_from(0.10975748638225852664e-1_f64) * t47929 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t9657 * t6918 * t4077 + F::cast_from(0.92526556154787137113e-2_f64) * t47932 - F::cast_from(0.43902994552903410656e-1_f64) * t47936 - F::cast_from(0.43902994552903410656e-1_f64) * t74744 + F::cast_from(0.520396828767081761e-2_f64) * t47938 + F::cast_from(0.2601984143835408805e-2_f64) * t47942 - F::cast_from(0.52039682876708176102e-1_f64) * t47944;
    t74749
}
