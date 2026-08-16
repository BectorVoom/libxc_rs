//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1020/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1020(t10073: f64, t2786: f64, t231: f64, t2783: f64, t836: f64, t860: f64, t2782: f64, t251: f64, t2645: f64, t10111: f64, t22: f64, t870: f64) -> (f64, f64, f64, f64) {
    let t10925 = t10073 * t2786;
    let t10929 = t2783 * t860 * t836 * t231;
    let t10930 = t2782 * t10929;
    let t10932 = t251 * t2645;
    let t10934 = t2783 * t10932 * t231;
    let t10935 = t2782 * t10934;
    let t10939 = 0.19637199382202157274e-3_f64 * t10111 * t870 * t22;
    (t10925, t10930, t10935, t10939)
}
