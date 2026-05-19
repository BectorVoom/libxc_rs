//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 758/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk758<F: Float>(t1127: F, t5025: F, t224: F, t9682: F, t17975: F, t1111: F, t1115: F, t13443: F, t13491: F, t1417: F, t1701: F, t17870: F, t21130: F, t21135: F, t21144: F, t21145: F, t2384: F, t3766: F, t3780: F, t4943: F, t5003: F, t5007: F, t5016: F, t5049: F, t807: F, t9524: F) -> (F, F, F) {
    let t21157 = t5025 * t1127;
    let t21159 = t224 * t9682 * t21157;
    let t21165 = t17975 * t1127;
    let t21171 = -F::cast_from(0.40559281352147498558e-4_f64) * t9524 * t21130 * t2384 + F::cast_from(0.41352194951222972388e-3_f64) * t17870 * t21135 - F::cast_from(0.33776098467676728323e-5_f64) * t807 * t21130 * t2384 + F::new(12.0) * t3766 * t13491 * t5025 - F::cast_from(0.82704389902445944777e-3_f64) * t21144 * t4943 * t21145 - F::cast_from(0.17782141943527538963e-1_f64) * t1417 * t1701 * t3780 * t5049 + F::cast_from(0.35564283887055077925e-1_f64) * t5007 * t1111 + F::cast_from(0.84321219226603029514e-3_f64) * t1115 * t5016 - F::new(6.0) * t21159 + F::cast_from(0.35564283887055077925e-1_f64) * t13443 * t1701 * t3780 * t5025 + F::cast_from(0.11262023230900774676e0_f64) * t1417 * t1701 * t21165 - F::cast_from(0.22524046461801549353e0_f64) * t1115 * t5003;
    (t21157, t21159, t21171)
}
