//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 841/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk841(t2020: f64, t31304: f64, t6997: f64, t8607: f64, t8562: f64, t865: f64, t2718: f64, t225: f64, t258: f64, t7084: f64, t214: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31305 = t31304 * t2020;
    let t31306 = t8607 * t6997;
    let t31310 = t8562 * t865;
    let t31311 = t2718 * t31310;
    let t31315 = t7084 * t225 * t258;
    let t31316 = t214 * t31315;
    let t31317 = t1880 * t31316;
    (t31305, t31306, t31311, t31315, t31316, t31317)
}
