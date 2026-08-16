//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 595/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk595(t1114: f64, t4670: f64, t345: f64, t2918: f64, t2919: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64, t261: f64, t1666: f64, t930: f64) -> (f64, f64, f64, f64, f64) {
    let t4671 = t1114 * t4670;
    let t4672 = t345 * t4671;
    let t4682 = t2918 + 0.5936111111111111111e-2_f64 * t2919 + 0.5936111111111111111e-2_f64 * t4612 - 0.11872222222222222222e-1_f64 * t4615 + 0.35616666666666666666e-1_f64 * t4618 - 0.35616666666666666666e-1_f64 * t4623;
    let t4684 = 0.62182e-1_f64 * t4682 * t261;
    let t4685 = t1666 * t930;
    (t4671, t4672, t4682, t4684, t4685)
}
