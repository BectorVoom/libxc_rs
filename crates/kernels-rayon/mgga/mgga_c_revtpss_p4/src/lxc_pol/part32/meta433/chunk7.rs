//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1548/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1548(t12712: f64, t471: f64, t6688: f64, t3720: f64, t1774: f64, t3367: f64, t4181: f64, t3626: f64, t6622: f64, t73: f64, t5352: f64, t20956: f64, t5333: f64) -> (f64, f64, f64, f64, f64) {
    let t21028 = t12712 * t471;
    let t21029 = t6688 * t21028;
    let t21030 = t3720 * t21029;
    let t21035 = t1774 * t3367;
    let t21036 = t21035 * t4181;
    let t21037 = t3626 * t21036;
    let t21040 = t6622 * t73;
    let t21041 = t21040 * t5352;
    let t21042 = t3720 * t21041;
    let t21045 = t20956 * t5333;
    (t21030, t21037, t21040, t21042, t21045)
}
