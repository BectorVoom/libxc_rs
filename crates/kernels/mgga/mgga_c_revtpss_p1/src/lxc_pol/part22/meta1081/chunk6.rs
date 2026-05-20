//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3900/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900<F: Float>(t13805: F, t1399: F, t14193: F, t21981: F, t22253: F, t22321: F, t3924: F, t4004: F, t4114: F, t4118: F, t47961: F, t47963: F, t47967: F, t47971: F, t5745: F, t5755: F, t73942: F, t74893: F, t74901: F, t74908: F, t74922: F, t820: F) -> F {
    let t74926 = -F::cast_from(0.19514881078765566038e-1_f64) * t74893 - F::cast_from(0.22089088168956307394e-3_f64) * t47961 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t4118 * t22253 + F::cast_from(0.11565819519348392139e-2_f64) * t74901 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t22321 * t3924 - F::cast_from(0.39029762157531132074e-1_f64) * t74908 + F::cast_from(0.26341796731742046394e1_f64) * t820 * t4114 * t73942 + F::cast_from(0.29268663035268940438e-1_f64) * t47963 - F::cast_from(0.39274398764404314548e-3_f64) * t47967 + F::cast_from(0.60712963356159538786e-1_f64) * t47971 - F::cast_from(0.79025390195226139182e1_f64) * t14193 * t21981 * t13805 + F::cast_from(0.79025390195226139182e1_f64) * t5745 * t21981 * t4004 - F::cast_from(0.26341796731742046394e1_f64) * t5755 * t74922 * t1399;
    t74926
}
