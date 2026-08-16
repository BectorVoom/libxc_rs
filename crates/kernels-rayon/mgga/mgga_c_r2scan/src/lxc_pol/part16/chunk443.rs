//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 443/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk443(t2161: f64, t2162: f64, t2164: f64, t133: f64, t774: f64, t255: f64) -> (f64, f64, f64) {
    let t2166 = 0.81312004494856525156e-4_f64 * t2161 * t2162 * t2164;
    let t2167 = t133 * t774;
    let t2168 = t2167 * t255;
    (t2166, t2167, t2168)
}
