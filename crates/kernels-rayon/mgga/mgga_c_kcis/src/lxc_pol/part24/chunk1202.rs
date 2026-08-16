//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1202/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1202(t26954: f64, t28203: f64, t7773: f64, t993: f64, t15573: f64, t28152: f64, t7788: f64, t95868: f64, t27055: f64, t28190: f64, t28204: f64, t27014: f64, t28160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96926 = t28203 * t26954;
    let t96935 = t993 * t7773;
    let t96940 = t15573 * t28152;
    let t96942 = 0.23168402777777777778e-3_f64 * t7788 * t96940;
    let t96943 = 0.15476481481481481481e-2_f64 * t95868;
    let t96945 = 0.23168402777777777778e-3_f64 * t28190 * t27055;
    let t96946 = t28204 * t27055;
    let t96952 = 0.23168402777777777778e-3_f64 * t27014 * t28160;
    (t96926, t96935, t96940, t96942, t96943, t96945, t96946, t96952)
}
