//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 947/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk947(t74021: f64, t74024: f64, t74027: f64, t74033: f64, t74036: f64, t74043: f64, t74046: f64, t70806: f64, t70809: f64, t70812: f64, t15492: f64, t2160: f64, t638: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76828 = 0.2553875993597870364e-4_f64 * t74021;
    let t76829 = 0.3830813990396805546e-4_f64 * t74024;
    let t76830 = 0.2553875993597870364e-4_f64 * t74027;
    let t76831 = 0.1276937996798935182e-4_f64 * t74033;
    let t76832 = 0.15961724959986689775e-4_f64 * t74036;
    let t76834 = 0.15961724959986689775e-4_f64 * t74043;
    let t76835 = 0.15961724959986689775e-4_f64 * t74046;
    let t76836 = 0.79828278012425390426e-1_f64 * t70806;
    let t76837 = 0.11974241701863808564e0_f64 * t70809;
    let t76838 = 0.79828278012425390426e-1_f64 * t70812;
    let t76840 = t638 * t2160 * t15492;
    (t76828, t76829, t76830, t76831, t76832, t76834, t76835, t76836, t76837, t76838, t76840)
}
