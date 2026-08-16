//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3299/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3299<F: Float>(t10995: F, t18312: F, t686: F, t72: F, t18804: F, t2470: F, t14489: F, t18324: F, t2765: F, t41037: F, t41038: F, t41049: F, t41052: F, t41056: F, t41060: F, t4474: F, t51196: F, t51199: F, t51203: F, t51207: F, t51211: F, t51213: F, t51216: F, t51227: F, t51231: F) -> F {
    let t62523 = t10995 * t18312 * t72 * t686;
    let t62528 = t10995 * t18804 * t2470;
    let t62545 = F::cast_from(0.78059524315062264149e-1_f64) * t62523 + t41037 + F::cast_from(0.26341796731742046394e1_f64) * t2765 * t18324 - F::cast_from(0.26019841438354088049e-1_f64) * t62528 + F::cast_from(0.2601984143835408805e-1_f64) * t41038 + F::cast_from(0.21951497276451705328e-1_f64) * t51196 + F::cast_from(0.2601984143835408805e-2_f64) * t51199 + t41049 - F::cast_from(0.2601984143835408805e-1_f64) * t41052 + F::cast_from(0.92526556154787137113e-2_f64) * t51203 + F::cast_from(0.92526556154787137113e-2_f64) * t51207 - F::cast_from(0.11565819519348392139e-2_f64) * t41056 + F::cast_from(0.60712963356159538786e-1_f64) * t51211 + F::cast_from(0.34146773541147097178e-1_f64) * t51213 + F::cast_from(0.2601984143835408805e-2_f64) * t51216 + F::cast_from(0.60712963356159538784e-1_f64) * t41060 + F::cast_from(0.10975748638225852664e-1_f64) * t51227 - F::cast_from(0.79025390195226139182e1_f64) * t4474 * t14489 - F::cast_from(0.39029762157531132076e-1_f64) * t51231;
    t62545
}
