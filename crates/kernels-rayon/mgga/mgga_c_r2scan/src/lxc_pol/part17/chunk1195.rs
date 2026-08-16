//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1195/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1195(t3591: f64, t39739: f64, t10760: f64, t2147: f64, t30304: f64, t3178: f64, t545: f64, t3300: f64, t3290: f64, t9302: f64, t12486: f64, t24039: f64) -> (f64, f64, f64, f64, f64) {
    let t43672 = t39739 * t3591;
    let t43677 = t2147 * t10760 * t30304;
    let t43681 = t545 * t3178;
    let t43682 = t43681 * t3300;
    let t43688 = t3290 * t9302;
    let t43690 = t24039 * t12486;
    (t43672, t43677, t43682, t43688, t43690)
}
