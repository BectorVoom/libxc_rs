//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1250/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1250(t19817: f64, t19818: f64, t1398: f64, t580: f64, t30: f64, t3724: f64, t1288: f64, t750: f64, t821: f64, t33: f64, t823: f64, t3683: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19819 = t19817 * t19818;
    let t19821 = t580 * t1398;
    let t19825 = t30 * t3724;
    let t19829 = t1288 * t750;
    let t19836 = t1288 * t821;
    let t20011 = t823 * t33;
    let t20012 = t20011 * t3683;
    (t19819, t19821, t19825, t19829, t19836, t20011, t20012)
}
