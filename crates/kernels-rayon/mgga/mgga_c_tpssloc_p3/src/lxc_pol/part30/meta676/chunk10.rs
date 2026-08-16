//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2117/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117(t1873: f64, t96657: f64, t28007: f64, t6534: f64, t26114: f64, t7467: f64, t26117: f64, t26135: f64, t7676: f64, t2314: f64, t28017: f64, t5113: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96659 = 2.0_f64 * t96657 * t1873;
    let t96661 = 2.0_f64 * t28007 * t6534;
    let t96663 = 4.0_f64 * t26114 * t7467;
    let t96665 = 4.0_f64 * t26117 * t7467;
    let t96667 = 4.0_f64 * t7676 * t26135;
    let t96669 = 2.0_f64 * t2314 * t28017;
    let t96671 = 2.0_f64 * t5113 * t28017;
    (t96659, t96661, t96663, t96665, t96667, t96669, t96671)
}
