//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 972/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk972(t1071: f64, t240: f64, t9: f64, t2866: f64, t990: f64, t2872: f64, t2881: f64, t2880: f64, t2900: f64, t991: f64, t109: f64, t992: f64) -> (f64, f64, f64, f64, f64) {
    let t9896 = 1.0_f64 / t240 / t1071;
    let t9897 = t9 * t9896;
    let t9903 = t2866 * t990;
    let t9906 = t2872 * t2881;
    let t9909 = t2880 * t2900;
    let t9910 = t991 * t9909;
    let t9916 = t109 * t992;
    (t9897, t9903, t9906, t9910, t9916)
}
