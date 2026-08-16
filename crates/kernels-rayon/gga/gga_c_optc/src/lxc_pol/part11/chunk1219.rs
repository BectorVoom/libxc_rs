//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1219/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1219(t28617: f64, t38318: f64, t28635: f64, t28637: f64, t22403: f64, t22625: f64, t22627: f64, t22636: f64, t28610: f64, t28626: f64, t28628: f64, t4733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56039 = 0.2077890707925103596e3_f64 * t28617;
    let t56040 = 0.35089340384731224426e1_f64 * t38318;
    let t56043 = 96.0_f64 * t28635;
    let t56044 = 576.0_f64 * t28637;
    let t56045 = t22625 + t22627 - 14.0_f64 * t28610 - t56039 - t56040 - 1820.0_f64 / 27.0_f64 * t28626 - 14.0_f64 * t28628 - t56043 - t56044 - t22403 - t22636;
    let t56047 = t4733 * t4733;
    (t56039, t56040, t56043, t56044, t56045, t56047)
}
