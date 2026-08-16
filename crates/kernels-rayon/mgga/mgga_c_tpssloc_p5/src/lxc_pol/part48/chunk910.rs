//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 910/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk910(t112867: f64, t22986: f64, t23270: f64, t2553: f64, t30622: f64, t2717: f64, t6662: f64, t1888: f64, t865: f64, t1880: f64, t23190: f64, t6553: f64, t6571: f64) -> (f64, f64, f64, f64) {
    let t112868 = 0.3289868133696452873e-1_f64 * t112867;
    let t112872 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t30622 * t2553;
    let t112873 = t2717 * t6662;
    let t112877 = 0.6579736267392905746e-1_f64 * t1888 * t23270 * t112873 * t865;
    let t112881 = 0.16449340668482264365e-1_f64 * t1880 * t6553 * t6571 * t23190;
    (t112868, t112872, t112877, t112881)
}
