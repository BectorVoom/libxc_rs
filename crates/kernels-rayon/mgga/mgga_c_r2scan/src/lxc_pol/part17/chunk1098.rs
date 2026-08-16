//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1098/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1098(t10710: f64, t25503: f64, t37658: f64, t11816: f64, t37880: f64, t10772: f64, t10810: f64, t2568: f64, t10768: f64, t8129: f64, t2604: f64, t625: f64) -> (f64, f64, f64, f64, f64) {
    let t39443 = t37658 * t10710 * t25503;
    let t39445 = t37880 * t11816;
    let t39458 = t10772 * t10810 * t2568;
    let t39464 = t10768 * t8129;
    let t39469 = t2604 * t625;
    (t39443, t39445, t39458, t39464, t39469)
}
