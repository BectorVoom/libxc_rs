//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1052/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1052(t12810: f64, t12856: f64, t3720: f64, t1250: f64, t12726: f64, t11772: f64, t3623: f64, t3717: f64, t3712: f64, t372: f64, t3630: f64, t12705: f64, t5341: f64) -> (f64, f64, f64, f64, f64) {
    let t12857 = t12810 * t12856;
    let t12858 = t3720 * t12857;
    let t12861 = t12726 * t1250;
    let t12862 = t3720 * t12861;
    let t12865 = t3623 * t11772;
    let t12866 = t3717 * t12865;
    let t12867 = t372 * t3712;
    let t12868 = t12867 * t3630;
    let t12871 = t12705 * t5341;
    (t12858, t12862, t12866, t12868, t12871)
}
