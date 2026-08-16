//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 294/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk294(t1634: f64, t583: f64, t573: f64, t574: f64) -> (f64, f64, f64, f64, f64) {
    let t1635 = 0.17808333333333333333e-1_f64 * t1634;
    let t1643 = t583 * t583;
    let t1644 = 1.0_f64 / t1643;
    let t1645 = t573 * t1644;
    let t1646 = 1.0_f64 / t574;
    (t1635, t1643, t1644, t1645, t1646)
}
