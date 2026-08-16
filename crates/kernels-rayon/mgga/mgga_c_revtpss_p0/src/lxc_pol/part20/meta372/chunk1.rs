//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1353/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353(t231: f64, t2760: f64, t2782: f64, t2783: f64, t836: f64, t10871: f64, t14545: f64, t39709: f64, t2645: f64, t234: f64, t39545: f64, t685: f64, t875: f64) -> (f64, f64, f64, f64) {
    let t40278 = t2782 * t2783 * t2760 * t836 * t231;
    let t40282 = t2782 * t14545 * t39709 * t10871;
    let t40284 = t10871 * t2645;
    let t40294 = 0.65457331274007190912e-5_f64 * t39545 * t234 * t875 * t685;
    (t40278, t40282, t40284, t40294)
}
