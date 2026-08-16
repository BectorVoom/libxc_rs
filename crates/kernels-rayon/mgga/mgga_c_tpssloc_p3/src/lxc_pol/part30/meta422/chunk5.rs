//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1626/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1626(t3640: f64, t6270: f64, t11947: f64, t6274: f64, t1254: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18837: f64, t18839: f64, t18917: f64, t18920: f64, t18922: f64, t18924: f64, t18928: f64, t18930: f64, t18932: f64, t18936: f64, t18938: f64, t4700: f64) -> f64 {
    let t19267 = t6270 * t3640;
    let t19270 = t6274 * t11947;
    let t19274 = -t1254 * t19267 * t4700 + 2.0_f64 * t1254 * t19270 * t4700 - t18682 - t18685 + t18688 + t18690 + t18692 - t18694 + t18696 + t18837 + t18839 - t18917 + t18920 + t18922 - t18924 - t18928 + t18930 + t18932 + t18936 - t18938;
    t19274
}
