//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 932/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk932(t74102: f64, t74161: f64, t74163: f64, t70867: f64, t74171: f64, t74173: f64, t74175: f64, t74177: f64, t74180: f64, t14588: f64, t623: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76859 = 0.2553875993597870364e-4_f64 * t74102;
    let t76878 = 0.23268647941669485538e-4_f64 * t74161;
    let t76879 = 0.11634323970834742769e-3_f64 * t74163;
    let t76880 = 0.29795219925308487579e-4_f64 * t70867;
    let t76884 = 0.1276937996798935182e-4_f64 * t74171;
    let t76885 = 0.2553875993597870364e-4_f64 * t74173;
    let t76886 = 0.3830813990396805546e-4_f64 * t74175;
    let t76887 = 0.1276937996798935182e-4_f64 * t74177;
    let t76888 = 0.1276937996798935182e-4_f64 * t74180;
    let t76890 = t623 * t14588;
    let t76891 = t76890 * t2147;
    (t76859, t76878, t76879, t76880, t76884, t76885, t76886, t76887, t76888, t76891)
}
