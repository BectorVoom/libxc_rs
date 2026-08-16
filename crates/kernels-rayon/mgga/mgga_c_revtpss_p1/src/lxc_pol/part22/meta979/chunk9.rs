//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3299/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3299(t10995: f64, t18312: f64, t686: f64, t72: f64, t18804: f64, t2470: f64, t14489: f64, t18324: f64, t2765: f64, t41037: f64, t41038: f64, t41049: f64, t41052: f64, t41056: f64, t41060: f64, t4474: f64, t51196: f64, t51199: f64, t51203: f64, t51207: f64, t51211: f64, t51213: f64, t51216: f64, t51227: f64, t51231: f64) -> f64 {
    let t62523 = t10995 * t18312 * t72 * t686;
    let t62528 = t10995 * t18804 * t2470;
    let t62545 = 0.78059524315062264149e-1_f64 * t62523 + t41037 + 0.26341796731742046394e1_f64 * t2765 * t18324 - 0.26019841438354088049e-1_f64 * t62528 + 0.2601984143835408805e-1_f64 * t41038 + 0.21951497276451705328e-1_f64 * t51196 + 0.2601984143835408805e-2_f64 * t51199 + t41049 - 0.2601984143835408805e-1_f64 * t41052 + 0.92526556154787137113e-2_f64 * t51203 + 0.92526556154787137113e-2_f64 * t51207 - 0.11565819519348392139e-2_f64 * t41056 + 0.60712963356159538786e-1_f64 * t51211 + 0.34146773541147097178e-1_f64 * t51213 + 0.2601984143835408805e-2_f64 * t51216 + 0.60712963356159538784e-1_f64 * t41060 + 0.10975748638225852664e-1_f64 * t51227 - 0.79025390195226139182e1_f64 * t4474 * t14489 - 0.39029762157531132076e-1_f64 * t51231;
    t62545
}
