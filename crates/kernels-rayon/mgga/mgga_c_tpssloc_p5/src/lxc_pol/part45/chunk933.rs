//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 933/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk933(t30642: f64, t6562: f64, t794: f64, t1902: f64, t213: f64, t225: f64, t22986: f64, t23272: f64, t23035: f64, t23241: f64, t30663: f64, t1880: f64, t82124: f64, t8335: f64) -> (f64, f64, f64, f64) {
    let t112892 = t6562 * t794 * t30642;
    let t112893 = 0.16449340668482264365e-1_f64 * t112892;
    let t112899 = t213 * t1902 * t225;
    let t112902 = 0.6579736267392905746e-1_f64 * t22986 * t112899 * t23272;
    let t112905 = 0.9869604401089358619e-1_f64 * t23035 * t30663 * t23241;
    let t112915 = 0.16449340668482264365e-1_f64 * t1880 * t82124 * t8335;
    (t112893, t112902, t112905, t112915)
}
