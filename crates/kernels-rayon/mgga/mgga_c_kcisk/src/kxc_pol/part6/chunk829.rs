//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 829/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk829(t3739: f64, t7836: f64, t8083: f64, t3748: f64, t8086: f64, t1333: f64, t8164: f64, t3924: f64, t8059: f64, t12841: f64, t8094: f64, t1219: f64, t7828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26869 = t3739 * t7836;
    let t26914 = t3739 * t8083;
    let t26919 = t3748 * t8086;
    let t26936 = t1333 * t8164;
    let t26992 = t8059 * t3924;
    let t27008 = t12841 * t8094;
    let t27016 = t7828 * t1219;
    (t26869, t26914, t26919, t26936, t26992, t27008, t27016)
}
