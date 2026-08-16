//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3917/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3917(t2782: f64, t4086: f64, t543: f64, t74965: f64, t4003: f64, t5744: f64, t74982: f64, t74700: f64, t4100: f64, t14122: f64, t21990: f64, t22005: f64, t3924: f64, t47450: f64, t47454: f64, t47455: f64, t49426: f64, t49429: f64, t49432: f64, t49446: f64, t49450: f64, t5735: f64, t5745: f64, t5755: f64, t74314: f64) -> f64 {
    let t75298 = t2782 * t4086 * t74965 * t543;
    let t75302 = t2782 * t5744 * t74982 * t4003;
    let t75305 = t74700 * t543;
    let t75307 = t2782 * t4100 * t75305;
    let t75324 = 0.10975748638225852664e-1_f64 * t75298 - 0.21951497276451705328e-1_f64 * t75302 - 0.2601984143835408805e-2_f64 * t49426 + 0.10975748638225852664e-1_f64 * t75307 + 0.2601984143835408805e-2_f64 * t49429 - 0.92526556154787137113e-2_f64 * t49432 - 0.92526556154787137112e-2_f64 * t47450 + t47454 + 0.52683593463484092788e1_f64 * t5745 * t14122 * t21990 - 0.19514881078765566038e-1_f64 * t49446 + 0.78059524315062264152e-1_f64 * t49450 - 0.65854491829355115987e0_f64 * t5755 * t22005 * t3924 - 0.52039682876708176102e-2_f64 * t47455 + 0.26341796731742046394e1_f64 * t5745 * t5735 * t74314;
    t75324
}
