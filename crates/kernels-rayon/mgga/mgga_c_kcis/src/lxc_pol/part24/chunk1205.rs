//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1205/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1205(t7772: f64, t96727: f64, t7794: f64, t993: f64, t2888: f64, t27028: f64, t15573: f64, t28178: f64, t7788: f64, t28183: f64, t11061: f64, t8090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97060 = 0.92754700520833333333e-4_f64 * t7772 * t96727;
    let t97083 = t993 * t7794;
    let t97089 = t2888 * t7794;
    let t97093 = t993 * t27028;
    let t97102 = 0.46336805555555555556e-3_f64 * t7788 * t15573 * t28178;
    let t97103 = t15573 * t28183;
    let t97105 = 0.23168402777777777778e-3_f64 * t7788 * t97103;
    let t97106 = t7772 * t97103;
    let t97153 = t7788 * t11061 * t8090;
    (t97060, t97083, t97089, t97093, t97102, t97105, t97106, t97153)
}
