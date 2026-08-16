//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1277/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1277(t27369: f64, t98847: f64, t16618: f64, t303: f64, t553: f64, t12231: f64, t6140: f64, t1014: f64, t28525: f64, t16761: f64, t28524: f64, t3955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98854 = 0.61836467013888888889e-4_f64 * t27369 * t98847;
    let t98856 = t303 * t553 * t16618;
    let t98860 = t12231 * t6140;
    let t98863 = t1014 * t28525;
    let t98864 = 0.33163888888888888888e-2_f64 * t98863;
    let t98866 = t303 * t553 * t16761;
    let t98869 = t303 * t28524 * t3955;
    (t98854, t98856, t98860, t98863, t98864, t98866, t98869)
}
