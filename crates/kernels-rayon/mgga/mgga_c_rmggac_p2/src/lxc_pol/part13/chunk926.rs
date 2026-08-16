//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 926/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk926(t1986: f64, t5251: f64, t675: f64, t2310: f64, t35277: f64, t1525: f64, t236: f64, t321: f64, t3352: f64, t7230: f64, t615: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t40365 = t675 * t1986 * t5251;
    let t40367 = t35277 * t2310;
    let t40372 = t7230 * t3352 * t236 * t1525 * t321;
    let t40377 = t7230 * t3352 * t236 * t615 * t833;
    (t40365, t40367, t40372, t40377)
}
