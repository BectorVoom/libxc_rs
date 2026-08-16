//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1107/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1107(t40951: f64, t40970: f64, t40976: f64, t35845: f64, t35848: f64, t40963: f64, t40966: f64, t40968: f64, t40973: f64, t40979: f64, t40981: f64, t40991: f64) -> f64 {
    let t44089 = 0.2927036860455597649e0_f64 * t40951;
    let t44093 = 0.10909864661698136692e0_f64 * t40970;
    let t44095 = 0.1454648621559751559e0_f64 * t40976;
    let t44101 = -t44089 - 0.17961362552795712846e0_f64 * t40963 + 0.72732431077987577947e-1_f64 * t40966 + 0.40911992481368012596e-1_f64 * t40968 - t44093 + 0.13637330827122670865e0_f64 * t40973 + t44095 + 0.40911992481368012595e-1_f64 * t40979 + 0.17961362552795712846e0_f64 * t40981 + 0.3193131120497015617e0_f64 * t35845 - 0.47896966807455234256e0_f64 * t35848 - 0.17961362552795712846e0_f64 * t40991;
    t44101
}
