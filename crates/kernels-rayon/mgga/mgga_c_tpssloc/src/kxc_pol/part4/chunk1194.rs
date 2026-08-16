//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1194/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1194(t12050: f64, t12091: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t19599: f64, t9780: f64, t9789: f64) -> (f64, f64, f64) {
    let t19677 = 12.0_f64 * t12050;
    let t19678 = 0.17315859105681463759e2_f64 * t12091;
    let t19679 = -t15898 + t9780 + t19599 + t12044 + t15911 - t12048 + t19677 - t15916 - t15917 - t12057 - t12059 + t15923 - t9789 + t12087 - t19678 - t12094;
    (t19677, t19678, t19679)
}
