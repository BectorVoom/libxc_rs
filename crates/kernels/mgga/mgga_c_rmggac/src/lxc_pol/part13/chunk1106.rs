//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1106/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1106<F: Float>(t40918: F, t40944: F, t40949: F, t305: F, t40909: F, t40911: F, t40913: F, t40922: F, t40925: F, t40930: F, t40934: F, t40938: F, t40946: F, t43903: F) -> F {
    let t44075 = F::new(0.10909864661698136692e0) * t40918;
    let t44083 = F::new(0.58540737209111952978e0) * t40944;
    let t44085 = F::new(0.87811105813667929469e0) * t40949;
    let t44086 = -F::new(0.40911992481368012596e0) * t40909 - F::new(0.43639458646792546769e0) * t40911 - F::new(0.40911992481368012596e-1) * t40913 - t44075 + F::new(0.72732431077987577947e0) * t40922 - F::new(0.16364796992547205038e0) * t40925 - F::new(0.81823984962736025191e-1) * t40930 + F::new(0.10909864661698136692e0) * t40934 + F::new(0.11974241701863808564e0) * t305 * t43903 - F::new(0.8980681276397856423e-1) * t40938 + t44083 - F::new(0.17961362552795712846e0) * t40946 - t44085;
    t44086
}
