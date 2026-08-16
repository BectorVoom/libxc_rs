//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 914/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk914(t2553: f64, t30676: f64, t6552: f64, t6637: f64, t22893: f64, t23164: f64, t30677: f64, t1902: f64, t22986: f64, t6646: f64, t776: f64, t829: f64) -> (f64, f64, f64) {
    let t112959 = 0.3289868133696452873e-1_f64 * t6552 * t6637 * t30676 * t2553;
    let t112961 = t23164 * t22893 * t30677;
    let t112962 = 0.3289868133696452873e-1_f64 * t112961;
    let t112967 = 0.6579736267392905746e-1_f64 * t22986 * t6646 * t1902 * t776 * t829;
    (t112959, t112962, t112967)
}
