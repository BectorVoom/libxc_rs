//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 822/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk822(t6936: f64, t7712: f64, t1814: f64, t2002: f64, t559: f64, t1827: f64, t6945: f64, t1831: f64, t6952: f64, t6915: f64, t6922: f64, t6935: f64, t6949: f64, t7706: f64, t7710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7713 = t6936 * t7712;
    let t7715 = t1814 * t2002;
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    let t7722 = -t6915 - t7706 / 48.0_f64 - t6922 - 0.12111826828242117256e-2_f64 * t7710 - t6935 - 0.20186378047070195427e-3_f64 * t7713 + t7716 / 1536.0_f64 - t7718 / 1536.0_f64 - t6949 - t7720 / 384.0_f64;
    (t7713, t7715, t7716, t7718, t7720, t7722)
}
