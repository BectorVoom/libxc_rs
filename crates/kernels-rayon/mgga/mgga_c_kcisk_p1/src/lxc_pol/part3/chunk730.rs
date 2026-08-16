//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 730/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk730(t10441: f64, t1876: f64, t4598: f64, t10487: f64, t708: f64, t4595: f64, t1648: f64, t4652: f64, t7028: f64, t1417: f64, t4686: f64, t4626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11325 = t1876 * t4598 * t10441;
    let t11328 = t708 * t10487;
    let t11330 = t4595 * t11328 * t10441;
    let t11334 = t708 * t1648 * t4652;
    let t11335 = t7028 * t11334;
    let t11338 = t1417 * t4686;
    let t11340 = t1417 * t4626;
    (t11325, t11330, t11334, t11335, t11338, t11340)
}
