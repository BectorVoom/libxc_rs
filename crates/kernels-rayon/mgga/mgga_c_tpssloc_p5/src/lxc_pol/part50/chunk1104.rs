//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1104/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1104(t32961: f64, t383: f64, t1058: f64, t1610: f64, t1920: f64, t30876: f64, t32928: f64, t32931: f64, t32935: f64, t32939: f64, t32944: f64, t353: f64, t6687: f64, t6797: f64, t8404: f64) -> (f64, f64) {
    let t32962 = t383 * t32961;
    let t32964 = t30876 + 0.54831135561607547883e-2_f64 * t6687 * t32928 - 0.16449340668482264365e-1_f64 * t6687 * t32931 + 0.16449340668482264365e-1_f64 * t6797 * t32935 + 0.16449340668482264365e-1_f64 * t1920 * t32939 + t1610 * t8404 + t1058 * t32944 + t353 * t32962;
    (t32962, t32964)
}
