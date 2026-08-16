//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 803/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk803(t11690: f64, t11693: f64, t11696: f64, t11698: f64, t11704: f64, t11707: f64, t11936: f64, t12399: f64, t240: f64, t567: f64, t564: f64, t1152: f64, t3477: f64) -> (f64, f64) {
    let t12401 = t12399 * t240 + t11690 - t11693 + t11696 - t11698 - t11704 + t11707 - t11936;
    let t12402 = t567 * t12401;
    let t12403 = t564 * t12402;
    let t12404 = t12403 / 16.0_f64;
    let t12405 = t1152 * t3477;
    (t12404, t12405)
}
