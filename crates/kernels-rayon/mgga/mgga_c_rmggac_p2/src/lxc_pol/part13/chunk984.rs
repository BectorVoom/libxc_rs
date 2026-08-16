//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 984/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk984(t2010: f64, t2012: f64, t5002: f64, t2019: f64, t2020: f64, t8850: f64, t1652: f64, t1971: f64, t495: f64, t515: f64, t7230: f64, t34944: f64, t40888: f64) -> (f64, f64, f64, f64) {
    let t41616 = t2010 * t2012 * t5002;
    let t41619 = t2019 * t2020 * t8850;
    let t41627 = t7230 * t1971 * t515 * t1652 * t495;
    let t41631 = t34944 * t40888;
    (t41616, t41619, t41627, t41631)
}
