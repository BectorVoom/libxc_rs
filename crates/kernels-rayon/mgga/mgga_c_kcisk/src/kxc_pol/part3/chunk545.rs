//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 545/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk545(t1900: f64, t4581: f64, t1869: f64, t3517: f64, t710: f64, t1879: f64, t3521: f64, t1417: f64, t1884: f64, t1889: f64, t579: f64, t695: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4582 = t4581 * t1900;
    let t4583 = t1869 * t4582;
    let t4586 = 0.21901432222222222222e-3_f64 * t3517 * t710;
    let t4587 = t3521 * t1879;
    let t4589 = t1417 * t1884;
    let t4591 = t1417 * t1889;
    let t4593 = t579 * t695;
    (t4582, t4583, t4586, t4587, t4589, t4591, t4593)
}
