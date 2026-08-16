//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1456/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1456(t8689: f64, t8944: f64, t24994: f64, t2105: f64, t8110: f64, t112: f64, t34175: f64, t105108: f64, t117672: f64, t120865: f64, t120867: f64, t120869: f64, t120871: f64, t1458: f64, t24972: f64, t27273: f64, t27276: f64, t27921: f64, t31284: f64, t671: f64, t7056: f64, t7956: f64, t8508: f64) -> (f64, f64, f64, f64) {
    let t123194 = t8689 * t8944;
    let t123198 = t8689 * t24994;
    let t124673 = t8110 * t2105;
    let t124676 = t34175 * t112;
    let t124687 = t120865 + t120867 + 0.135e2_f64 * t27921 * t7056 + t31284 + t8508 + 0.135e2_f64 * t124676 * t671 + t120869 + t120871 + 27.0_f64 * t105108 * t7956 + 27.0_f64 * t24972 * t27273 + 27.0_f64 * t24972 * t27276 + 0.135e2_f64 * t117672 * t1458;
    (t123194, t123198, t124673, t124687)
}
