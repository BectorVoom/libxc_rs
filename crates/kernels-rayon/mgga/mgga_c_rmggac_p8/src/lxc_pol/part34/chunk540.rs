//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 540/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk540(t14327: f64, t793: f64, t3065: f64, t6444: f64, t13988: f64, t5259: f64, t13992: f64, t4669: f64, t14078: f64, t2500: f64, t14102: f64, t3075: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14328 = t793 * t14327;
    let t14330 = t6444 * t3065;
    let t14333 = 0.5987120850931904282e-1_f64 * t5259 * t13988;
    let t14335 = 0.8980681276397856423e-1_f64 * t4669 * t13992;
    let t14336 = t2500 * t14078;
    let t14338 = t3075 * t14102;
    (t14328, t14330, t14333, t14335, t14336, t14338)
}
