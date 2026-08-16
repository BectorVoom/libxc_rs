//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 907/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk907(t2157: f64, t5011: f64, t2350: f64, t866: f64, t262: f64, t7192: f64, t848: f64, t8630: f64, t833: f64, t7198: f64, t333: f64, t8708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39678 = t5011 * t2157;
    let t39679 = 0.79828278012425390426e-1_f64 * t39678;
    let t39680 = t2350 * t866;
    let t39681 = t262 * t39680;
    let t39682 = t7192 * t39681;
    let t39684 = t2350 * t848;
    let t39685 = t262 * t39684;
    let t39686 = t8630 * t39685;
    let t39688 = t2350 * t833;
    let t39689 = t262 * t39688;
    let t39690 = t7198 * t39689;
    let t39692 = t8708 * t333;
    (t39679, t39680, t39681, t39682, t39684, t39685, t39686, t39688, t39689, t39690, t39692)
}
