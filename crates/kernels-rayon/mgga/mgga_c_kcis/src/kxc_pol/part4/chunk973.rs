//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 973/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk973(t9916: f64, t995: f64, t991: f64, t2909: f64, t993: f64, t1000: f64, t2888: f64, t2880: f64, t2895: f64, t2904: f64, t24: f64, t2887: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9917 = t9916 * t995;
    let t9918 = t991 * t9917;
    let t9924 = t993 * t2909;
    let t9933 = t2888 * t1000;
    let t9938 = t2880 * t1000;
    let t9939 = t9938 * t2895;
    let t9940 = t991 * t9939;
    let t9956 = t2880 * t2904;
    let t9957 = t991 * t9956;
    let t9959 = t24 * t2887;
    (t9918, t9924, t9933, t9938, t9940, t9957, t9959)
}
