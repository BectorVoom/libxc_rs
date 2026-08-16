//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1002/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1002(t33: f64, t7782: f64, t1711: f64, t1940: f64, t2403: f64, t31863: f64, t31876: f64, t33727: f64, t7091: f64, t7862: f64, t7869: f64, t8490: f64, t8494: f64) -> (f64, f64) {
    let t33888 = t33 * t7782;
    let t33896 = 3.0_f64 / 2.0_f64 * t2403 * t8490 * t7862 + t1940 * t33727 * t33 / 2.0_f64 - t1940 * t31863 * t7869 / 2.0_f64 + t1940 * t8490 * t1711 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t7862 - t1940 * t7091 * t33888 + t1940 * t31876 * t7869 - t1940 * t8494 * t1711 / 2.0_f64;
    (t33888, t33896)
}
