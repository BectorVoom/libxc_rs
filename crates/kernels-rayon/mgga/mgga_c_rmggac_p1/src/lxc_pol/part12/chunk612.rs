//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 612/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk612(t7828: f64, t7883: f64, t82: f64, t72: f64, t2150: f64, t504: f64, t2127: f64, t302: f64, t2025: f64, t4965: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7884 = t7828 + t7883;
    let t7885 = t82 * t7884;
    let t7886 = t72 * t7885;
    let t7887 = t504 * t2150;
    let t7888 = 0.39914139006212695214e-1_f64 * t7887;
    let t7889 = t302 * t2127;
    let t7890 = t72 * t7889;
    let t7891 = 2.0_f64 * t7890;
    let t7892 = t4965 * t2025;
    let t7893 = 0.79828278012425390428e-1_f64 * t7892;
    let t7894 = t290 * t2127;
    (t7884, t7885, t7886, t7888, t7889, t7891, t7893, t7894)
}
