//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 934/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk934(t225: f64, t26732: f64, t26734: f64, t27137: f64, t27059: f64, t27070: f64, t27052: f64, t2085: f64, t5286: f64, t1824: f64, t7191: f64, t112: f64, t27240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92847 = t26732 * t225;
    let t92939 = t26734 * t225;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93338 = t27070 * t225;
    let t93341 = t27052 * t225;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t94127 = t27240 * t112;
    (t92847, t92939, t93313, t93316, t93338, t93341, t93501, t93505, t94127)
}
