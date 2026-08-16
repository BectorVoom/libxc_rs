//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 933/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk933(t1096: f64, t8507: f64, t31959: f64, t1052: f64, t359: f64, t369: f64, t8499: f64, t11921: f64, t247: f64, t385: f64, t8502: f64, t1982: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31960 = t8507 * t1096;
    let t31961 = t31959 * t31960;
    let t31964 = t359 * t1052;
    let t31965 = t31964 * t369;
    let t31966 = t8499 * t31965;
    let t31970 = t247 * t11921 * t385;
    let t31972 = 0.18822977838986977999e-3_f64 * t8502 * t31970;
    let t31973 = t1982 * t3140;
    (t31961, t31964, t31965, t31966, t31970, t31972, t31973)
}
