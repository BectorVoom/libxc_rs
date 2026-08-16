//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 986/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk986(t3671: f64, t8313: f64, t10590: f64, t2175: f64, t2177: f64, t3629: f64, t8320: f64, t3628: f64, t3630: f64, t8330: f64, t1385: f64, t8130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10600 = 7.0_f64 / 2304.0_f64 * t8313 * t3671;
    let t10602 = t2175 * t10590 * t2177;
    let t10606 = t2175 * t3629 * t8320;
    let t10610 = t3628 * t10590 * t3630;
    let t10614 = t3628 * t3629 * t8330;
    let t10617 = t8130 * t1385;
    (t10600, t10602, t10606, t10610, t10614, t10617)
}
