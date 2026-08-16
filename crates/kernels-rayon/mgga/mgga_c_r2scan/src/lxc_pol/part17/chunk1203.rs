//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1203/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1203(t481: f64, t9573: f64, t2847: f64, t3582: f64, t2333: f64, t3016: f64, t795: f64, t12043: f64, t12721: f64, t12723: f64, t12726: f64, t12728: f64, t12730: f64, t12733: f64, t41116: f64, t41117: f64, t41118: f64, t41119: f64, t41120: f64, t41121: f64, t41122: f64, t41123: f64) -> (f64, f64, f64, f64) {
    let t43959 = t9573 * t481;
    let t43979 = t3582 * t2847;
    let t43983 = t2333 * t3016;
    let t43984 = t43983 * t795;
    let t44008 = -t12721 + t41116 - t41117 + t41118 - t41119 - t41120 + t41121 + t12723 + t12726 + t12728 + t41122 + t12043 + t12730 + t12733 - t41123;
    (t43959, t43979, t43984, t44008)
}
