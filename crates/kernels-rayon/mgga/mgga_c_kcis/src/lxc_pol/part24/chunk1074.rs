//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1074/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1074(t28040: f64, t28070: f64, t1142: f64, t1291: f64, t8117: f64, t1872: f64, t7823: f64, t15573: f64, t8086: f64) -> (f64, f64, f64, f64, f64) {
    let t28071 = t28040 + t28070;
    let t28072 = t1142 * t28071;
    let t28073 = t8117 * t1291;
    let t28076 = t7823 * t1872;
    let t28093 = t15573 * t8086;
    (t28071, t28072, t28073, t28076, t28093)
}
