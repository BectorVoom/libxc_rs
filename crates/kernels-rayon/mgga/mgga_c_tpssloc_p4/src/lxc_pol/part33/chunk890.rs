//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 890/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk890(t3315: f64, t6020: f64, t1222: f64, t6170: f64, t6158: f64, t6165: f64, t5416: f64, t972: f64, t135: f64, t6187: f64, t1174: f64, t4889: f64, t5040: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18258 = t6020 * t3315;
    let t18310 = t6170 * t1222;
    let t18312 = t6158 * t1222;
    let t18314 = t6165 * t1222;
    let t18321 = t5416 * t972;
    let t18324 = t135 * t6187;
    let t18325 = t1174 * t18324;
    let t18327 = t4889 * t5040;
    (t18258, t18310, t18312, t18314, t18321, t18325, t18327)
}
