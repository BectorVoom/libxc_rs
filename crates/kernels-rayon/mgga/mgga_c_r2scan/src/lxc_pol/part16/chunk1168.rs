//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1168/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1168(t11554: f64, t2850: f64, t3275: f64, t3276: f64, t11555: f64, t11629: f64, t1100: f64, t42929: f64, t42931: f64, t42933: f64, t42937: f64, t42939: f64, t42943: f64, t42947: f64, t42949: f64, t42951: f64, t42953: f64, t42958: f64, t42962: f64, t42965: f64, t9832: f64) -> (f64, f64, f64) {
    let t42966 = t11554 * t2850;
    let t42969 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t42966;
    let t42972 = 5.0_f64 / 8.0_f64 * t3275 * t11629 * t11555;
    let t42973 = t1100 * t9832 - t42929 - t42931 + t42933 + t42937 - t42939 + t42943 + t42947 - t42949 - t42951 - t42953 - t42958 - t42962 + t42965 + t42969 + t42972;
    (t42969, t42972, t42973)
}
