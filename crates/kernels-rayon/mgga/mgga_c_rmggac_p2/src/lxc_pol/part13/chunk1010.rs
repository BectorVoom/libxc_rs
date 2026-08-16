//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1010/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1010(t1614: f64, t2084: f64, t2139: f64, t27: f64, t3351: f64, t3352: f64, t511: f64, t5187: f64, t1175: f64, t236: f64, t618: f64, t7231: f64, t8517: f64) -> (f64, f64, f64) {
    let t42132 = t2139 * t27 * t2084 * t1614;
    let t42136 = t3351 * t3352 * t511 * t5187;
    let t42142 = t8517 * t7231 * t236 * t618 * t1175;
    (t42132, t42136, t42142)
}
