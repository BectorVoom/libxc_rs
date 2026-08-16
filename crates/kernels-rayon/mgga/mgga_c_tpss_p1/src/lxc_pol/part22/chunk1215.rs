//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1215/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1215(t1163: f64, t5815: f64, t508: f64, t5935: f64, t5709: f64, t18295: f64, t1845: f64, t18551: f64, t5909: f64, t1811: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18707 = t1163 * t5815;
    let t18710 = t508 * t5935;
    let t18711 = t18710 * t5709;
    let t18714 = t1845 * t18295;
    let t18717 = t5909 * t18551;
    let t18728 = t198 * t206 * t1811;
    (t18707, t18710, t18711, t18714, t18717, t18728)
}
