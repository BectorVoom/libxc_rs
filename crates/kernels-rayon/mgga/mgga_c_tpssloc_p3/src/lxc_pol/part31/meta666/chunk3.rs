//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1958/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958(t112: f64, t29395: f64, t100990: f64, t100993: f64, t12524: f64, t1401: f64, t1458: f64, t16524: f64, t19534: f64, t20176: f64, t24462: f64, t24465: f64, t27170: f64, t27273: f64, t27276: f64, t28951: f64, t29422: f64, t29425: f64, t33185: f64, t3938: f64, t5371: f64, t5376: f64, t5456: f64, t5493: f64, t55388: f64, t671: f64, t7230: f64, t7235: f64, t75795: f64, t7956: f64, t94127: f64, t94170: f64) -> f64 {
    let t100996 = t29395 * t112;
    let t101021 = 0.135e2_f64 * t7230 * t19534 + 0.135e2_f64 * t24462 * t5493 + 27.0_f64 * t55388 * t7235 + 0.135e2_f64 * t1401 * t100990 + 27.0_f64 * t100993 * t5456 + 0.135e2_f64 * t100996 * t671 + 54.0_f64 * t94170 * t5376 + 54.0_f64 * t75795 * t7956 + 54.0_f64 * t16524 * t27273 + 27.0_f64 * t5371 * t27170 + 0.135e2_f64 * t3938 * t28951 + 27.0_f64 * t94127 * t1458 + 27.0_f64 * t12524 * t29425 + 54.0_f64 * t33185 * t27276 + 54.0_f64 * t12524 * t29422 + 54.0_f64 * t16524 * t27276 + 54.0_f64 * t24465 * t20176;
    t101021
}
