//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1072/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1072(t70208: f64, t14434: f64, t70194: f64, t70198: f64, t71727: f64, t739: f64, t75968: f64, t78394: f64, t78395: f64, t78397: f64, t78399: f64, t78400: f64, t78401: f64, t78402: f64, t78403: f64, t78404: f64, t78405: f64, t78406: f64, t8377: f64) -> f64 {
    let t78409 = 0.79808624799933448875e-4_f64 * t70208;
    let t78413 = 0.72714524817717142308e-5_f64 * t75968 - t78394 + t78395 + t78397 + t78399 - t78400 - t78401 - t78402 - t78403 + t78404 - t78405 + t78406 - t71727 + 0.16566831523319392755e-1_f64 * t70194 + 0.82834157616596963775e-1_f64 * t70198 - t78409 + 0.11974241701863808564e0_f64 * t739 * t14434 * t8377;
    t78413
}
