//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1020/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1020(t1550: f64, t30800: f64, t7577: f64, t30490: f64, t903: f64, t35972: f64, t45556: f64, t739: f64, t35584: f64, t35587: f64, t35591: f64, t35593: f64, t40121: f64, t40124: f64, t40126: f64, t46034: f64, t46038: f64, t46040: f64, t46043: f64, t46045: f64, t46071: f64, t46322: f64, t46352: f64, t46372: f64, t46411: f64, t46449: f64, t46478: f64, t46518: f64, t46562: f64, t46601: f64, t46633: f64, t46666: f64, t46701: f64, t46734: f64, t46763: f64, t46786: f64, t5928: f64, t72: f64, t82: f64, t8801: f64) -> f64 {
    let t46800 = t1550 * t7577 * t30800;
    let t46803 = t903 * t7577 * t30490;
    let t46806 = t739 * t35972 * t45556;
    let t46808 = -0.25538759935978703638e-4_f64 * t46034 - 0.25538759935978703638e-4_f64 * t46038 - 0.5987120850931904282e-1_f64 * t46040 + 0.8980681276397856423e-1_f64 * t46043 + 0.2993560425465952141e-1_f64 * t46045 + t72 * t82 * (t46071 + t46322 + t46352 + t46372 + t46411 + t46449 + t46478 + t46518 + t46562 + t46601 + t46633 + t46666 + t46701 + t46734 + t46763 + t46786) + 0.59590439850616975157e-4_f64 * t40121 + t40124 + t40126 - 0.2927036860455597649e0_f64 * t35584 + 0.43905552906833964735e0_f64 * t35587 + 0.14635184302277988245e0_f64 * t35591 + t35593 + 0.79828278012425390428e-1_f64 * t5928 * t8801 - 0.5987120850931904282e-1_f64 * t46800 + 0.8980681276397856423e-1_f64 * t46803 + 0.8980681276397856423e-1_f64 * t46806;
    t46808
}
