//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 327/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk327(t695: f64, t708: f64, t574: f64, t682: f64, t707: f64, t642: f64, t654: f64) -> (f64, f64, f64, f64) {
    let t1877 = t708 * t695;
    let t1882 = t574 * t708;
    let t1887 = t707 * t682;
    let t1899 = t654 * t642;
    (t1877, t1882, t1887, t1899)
}
