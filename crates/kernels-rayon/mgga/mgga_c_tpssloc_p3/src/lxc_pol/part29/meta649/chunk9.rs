//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2167/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2167(t89775: f64, t89822: f64, t23788: f64, t59580: f64, t86815: f64, t13196: f64, t25891: f64, t25927: f64, t58009: f64, t10143: f64, t1081: f64, t25374: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89823 = t89775 + t89822;
    let t89837 = t23788 * t59580;
    let t89840 = t23788 * t86815;
    let t89843 = t25891 * t13196;
    let t89846 = t25927 * t58009;
    let t89849 = t10143 * t1081;
    let t89850 = t89849 * t25374;
    (t89823, t89837, t89840, t89843, t89846, t89850)
}
