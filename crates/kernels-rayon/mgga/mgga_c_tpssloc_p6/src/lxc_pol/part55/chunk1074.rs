//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1074/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1074(t32465: f64, t7375: f64, t2147: f64, t7348: f64, t462: f64, t1215: f64, t8882: f64, t1246: f64, t32451: f64, t493: f64, t1201: f64, t1244: f64, t2121: f64, t32456: f64, t32459: f64, t32462: f64, t470: f64, t7283: f64, t7373: f64, t8895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32466 = t7375 * t32465;
    let t32469 = t2147 * t7348;
    let t32470 = t462 * t32469;
    let t32474 = t8882 * t1215;
    let t32475 = t32474 * t1246;
    let t32477 = t493 * t32451;
    let t32479 = t32456 - 0.54831135561607547883e-2_f64 * t7283 * t32459 - 0.16449340668482264365e-1_f64 * t7283 * t32462 + 0.16449340668482264365e-1_f64 * t7373 * t32466 + 0.16449340668482264365e-1_f64 * t2121 * t32470 + t1201 * t8895 + t1244 * t32475 + t470 * t32477;
    (t32466, t32469, t32470, t32475, t32477, t32479)
}
