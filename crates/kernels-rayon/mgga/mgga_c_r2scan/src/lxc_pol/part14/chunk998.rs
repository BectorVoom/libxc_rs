//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 998/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk998(t1584: f64, t3597: f64, t3308: f64, t8066: f64, t574: f64, t2651: f64, t3309: f64, t10810: f64, t2608: f64, t10698: f64, t3588: f64, t1010: f64, t11033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11835 = t1584 * t3597;
    let t11837 = t3308 * t8066;
    let t11838 = t574 * t11837;
    let t11840 = t2651 * t3309;
    let t11842 = t10810 * t2608;
    let t11843 = t574 * t11842;
    let t11845 = t10698 * t3588;
    let t11866 = t11033 * t1010;
    (t11835, t11837, t11838, t11840, t11842, t11843, t11845, t11866)
}
