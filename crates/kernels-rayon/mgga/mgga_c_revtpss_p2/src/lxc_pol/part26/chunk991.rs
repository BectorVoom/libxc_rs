//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 991/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk991(t12730: f64, t12731: f64, t1287: f64, t487: f64, t12646: f64, t1280: f64, t1269: f64, t3588: f64, t1204: f64, t3781: f64, t1214: f64, t1209: f64, t5462: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12732 = t12730 + t12731;
    let t12734 = t487 * t12732 * t1287;
    let t12737 = t1280 * t12646;
    let t12741 = t1269 * t3588 * t1287;
    let t12744 = t1204 * t3781;
    let t12747 = t1214 * t3588;
    let t12748 = t12747 * t1287;
    let t12751 = t1209 * t5462;
    (t12732, t12734, t12737, t12741, t12744, t12748, t12751)
}
