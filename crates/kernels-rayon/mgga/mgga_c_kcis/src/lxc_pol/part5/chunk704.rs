//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 704/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk704(t5081: f64, t5188: f64, t1142: f64, t2919: f64, t3537: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64, t1211: f64, t1823: f64, t1219: f64, t1831: f64) -> (f64, f64, f64, f64, f64) {
    let t5189 = t5081 + t5188;
    let t5190 = t1142 * t5189;
    let t5208 = t3537 + 0.57077777777777777777e-2_f64 * t2919 + 0.57077777777777777777e-2_f64 * t4612 - 0.11415555555555555555e-1_f64 * t4615 + 0.34246666666666666666e-1_f64 * t4618 - 0.34246666666666666666e-1_f64 * t4623;
    let t5211 = t1823 * t1211;
    let t5216 = t1831 * t1219;
    (t5189, t5190, t5208, t5211, t5216)
}
