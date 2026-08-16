//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 782/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk782(t1975: f64, t5392: f64, t1973: f64, t5400: f64, t1980: f64, t4781: f64, t4790: f64, t1683: f64, t12019: f64, t1974: f64, t1670: f64, t4761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12066 = t1975 * t5392;
    let t12070 = t5392 * t5400 * t1973;
    let t12073 = t1980 * t4781;
    let t12076 = t4781 * t4790;
    let t12077 = t12076 * t1683;
    let t12081 = t12019 * t1974;
    let t12084 = t1670 * t4761;
    (t12066, t12070, t12073, t12077, t12081, t12084)
}
