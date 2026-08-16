//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1335/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1335(t12836: f64, t19469: f64, t215: f64, t12841: f64, t18464: f64, t4480: f64, t12838: f64, t5728: f64, t12843: f64, t12877: f64, t18454: f64, t1642: f64, t60706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65611 = t19469 * t215 * t12836;
    let t65614 = t19469 * t215 * t12841;
    let t65616 = t18464 * t4480;
    let t65618 = t5728 * t12838;
    let t65620 = t5728 * t12843;
    let t65622 = t18454 * t12877;
    let t65624 = t60706 * t1642;
    (t65611, t65614, t65616, t65618, t65620, t65622, t65624)
}
