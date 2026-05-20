//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3917/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3917<F: Float>(t2782: F, t4086: F, t543: F, t74965: F, t4003: F, t5744: F, t74982: F, t74700: F, t4100: F, t14122: F, t21990: F, t22005: F, t3924: F, t47450: F, t47454: F, t47455: F, t49426: F, t49429: F, t49432: F, t49446: F, t49450: F, t5735: F, t5745: F, t5755: F, t74314: F) -> F {
    let t75298 = t2782 * t4086 * t74965 * t543;
    let t75302 = t2782 * t5744 * t74982 * t4003;
    let t75305 = t74700 * t543;
    let t75307 = t2782 * t4100 * t75305;
    let t75324 = F::cast_from(0.10975748638225852664e-1_f64) * t75298 - F::cast_from(0.21951497276451705328e-1_f64) * t75302 - F::cast_from(0.2601984143835408805e-2_f64) * t49426 + F::cast_from(0.10975748638225852664e-1_f64) * t75307 + F::cast_from(0.2601984143835408805e-2_f64) * t49429 - F::cast_from(0.92526556154787137113e-2_f64) * t49432 - F::cast_from(0.92526556154787137112e-2_f64) * t47450 + t47454 + F::cast_from(0.52683593463484092788e1_f64) * t5745 * t14122 * t21990 - F::cast_from(0.19514881078765566038e-1_f64) * t49446 + F::cast_from(0.78059524315062264152e-1_f64) * t49450 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t22005 * t3924 - F::cast_from(0.52039682876708176102e-2_f64) * t47455 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t5735 * t74314;
    t75324
}
