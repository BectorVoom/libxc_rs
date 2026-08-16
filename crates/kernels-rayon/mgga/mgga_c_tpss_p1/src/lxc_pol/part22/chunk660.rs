//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 660/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk660(t1015: f64, t1114: f64, t3068: f64, t2856: f64, t2859: f64, t2866: f64, t2908: f64, t2916: f64, t3006: f64, t3008: f64, t3011: f64, t3015: f64, t3019: f64, t3023: f64) -> (f64, f64, f64) {
    let t3069 = t1114 * t1015;
    let t3070 = t3068 * t3069;
    let t3073 = -t2856 + t2859 - t2866 + t2908 + t2916 + t3006 + t3008 - t3011 + t3015 - t3019 - t3023;
    (t3069, t3070, t3073)
}
