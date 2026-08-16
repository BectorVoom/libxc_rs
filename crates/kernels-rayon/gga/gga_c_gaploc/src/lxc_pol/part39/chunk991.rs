//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 991/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk991(t2558: f64, t32743: f64, t9647: f64, t1841: f64, t3487: f64, t6118: f64, t9755: f64, t13063: f64, t2536: f64, t734: f64, t10809: f64, t161: f64, t9744: f64) -> (f64, f64, f64, f64) {
    let t43027 = t9647 * t32743 * t2558;
    let t43028 = 0.64087718584518535698e-3_f64 * t43027;
    let t43032 = 0.59815204012217299984e-2_f64 * t1841 * t9755 * t3487 * t6118;
    let t43035 = t1841 * t2536 * t13063 * t734;
    let t43040 = 0.10254034973522965711e-1_f64 * t1841 * t10809 * t161 * t9744;
    (t43028, t43032, t43035, t43040)
}
