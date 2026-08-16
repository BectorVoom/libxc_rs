//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1106/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1106(t40918: f64, t40944: f64, t40949: f64, t305: f64, t40909: f64, t40911: f64, t40913: f64, t40922: f64, t40925: f64, t40930: f64, t40934: f64, t40938: f64, t40946: f64, t43903: f64) -> f64 {
    let t44075 = 0.10909864661698136692e0_f64 * t40918;
    let t44083 = 0.58540737209111952978e0_f64 * t40944;
    let t44085 = 0.87811105813667929469e0_f64 * t40949;
    let t44086 = -0.40911992481368012596e0_f64 * t40909 - 0.43639458646792546769e0_f64 * t40911 - 0.40911992481368012596e-1_f64 * t40913 - t44075 + 0.72732431077987577947e0_f64 * t40922 - 0.16364796992547205038e0_f64 * t40925 - 0.81823984962736025191e-1_f64 * t40930 + 0.10909864661698136692e0_f64 * t40934 + 0.11974241701863808564e0_f64 * t305 * t43903 - 0.8980681276397856423e-1_f64 * t40938 + t44083 - 0.17961362552795712846e0_f64 * t40946 - t44085;
    t44086
}
