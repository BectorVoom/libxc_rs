//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3900/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900(t13805: f64, t1399: f64, t14193: f64, t21981: f64, t22253: f64, t22321: f64, t3924: f64, t4004: f64, t4114: f64, t4118: f64, t47961: f64, t47963: f64, t47967: f64, t47971: f64, t5745: f64, t5755: f64, t73942: f64, t74893: f64, t74901: f64, t74908: f64, t74922: f64, t820: f64) -> f64 {
    let t74926 = -0.19514881078765566038e-1_f64 * t74893 - 0.22089088168956307394e-3_f64 * t47961 - 0.13170898365871023197e1_f64 * t820 * t4118 * t22253 + 0.11565819519348392139e-2_f64 * t74901 - 0.65854491829355115987e0_f64 * t820 * t22321 * t3924 - 0.39029762157531132074e-1_f64 * t74908 + 0.26341796731742046394e1_f64 * t820 * t4114 * t73942 + 0.29268663035268940438e-1_f64 * t47963 - 0.39274398764404314548e-3_f64 * t47967 + 0.60712963356159538786e-1_f64 * t47971 - 0.79025390195226139182e1_f64 * t14193 * t21981 * t13805 + 0.79025390195226139182e1_f64 * t5745 * t21981 * t4004 - 0.26341796731742046394e1_f64 * t5755 * t74922 * t1399;
    t74926
}
