//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 758/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk758(t1127: f64, t5025: f64, t224: f64, t9682: f64, t17975: f64, t1111: f64, t1115: f64, t13443: f64, t13491: f64, t1417: f64, t1701: f64, t17870: f64, t21130: f64, t21135: f64, t21144: f64, t21145: f64, t2384: f64, t3766: f64, t3780: f64, t4943: f64, t5003: f64, t5007: f64, t5016: f64, t5049: f64, t807: f64, t9524: f64) -> (f64, f64, f64) {
    let t21157 = t5025 * t1127;
    let t21159 = t224 * t9682 * t21157;
    let t21165 = t17975 * t1127;
    let t21171 = -0.40559281352147498558e-4_f64 * t9524 * t21130 * t2384 + 0.41352194951222972388e-3_f64 * t17870 * t21135 - 0.33776098467676728323e-5_f64 * t807 * t21130 * t2384 + 12.0_f64 * t3766 * t13491 * t5025 - 0.82704389902445944777e-3_f64 * t21144 * t4943 * t21145 - 0.17782141943527538963e-1_f64 * t1417 * t1701 * t3780 * t5049 + 0.35564283887055077925e-1_f64 * t5007 * t1111 + 0.84321219226603029514e-3_f64 * t1115 * t5016 - 6.0_f64 * t21159 + 0.35564283887055077925e-1_f64 * t13443 * t1701 * t3780 * t5025 + 0.11262023230900774676e0_f64 * t1417 * t1701 * t21165 - 0.22524046461801549353e0_f64 * t1115 * t5003;
    (t21157, t21159, t21171)
}
