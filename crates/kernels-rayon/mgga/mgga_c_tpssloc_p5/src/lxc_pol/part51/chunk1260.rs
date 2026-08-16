//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1260/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1260(t225: f64, t27137: f64, t27059: f64, t2091: f64, t40590: f64, t27070: f64, t27052: f64, t2085: f64, t5286: f64, t1824: f64, t7191: f64, t12020: f64, t7213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93319 = t40590 * t2091;
    let t93338 = t27070 * t225;
    let t93341 = t27052 * t225;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t93818 = t12020 * t7213;
    (t93313, t93316, t93319, t93338, t93341, t93501, t93505, t93818)
}
