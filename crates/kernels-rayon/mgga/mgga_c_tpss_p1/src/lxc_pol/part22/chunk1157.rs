//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1157/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1157(t12920: f64, t2206: f64, t4438: f64, t10039: f64, t10028: f64, t10038: f64, t10042: f64, t12907: f64, t12909: f64, t12911: f64, t12913: f64, t12915: f64, t12918: f64, t12919: f64, t7979: f64, t7988: f64, t7992: f64) -> (f64, f64, f64, f64) {
    let t12921 = 0.17315859105681463759e2_f64 * t12920;
    let t12922 = t4438 * t2206;
    let t12923 = 0.5848223622634646207e0_f64 * t12922;
    let t12924 = 4.0_f64 * t10039;
    let t12925 = -t10028 - t12907 + t7979 + t12909 - t12911 + t12913 - t12915 + t12918 + t12919 - t12921 - t12923 - t10038 + t12924 - t10042 + t7988 + t7992;
    (t12921, t12923, t12924, t12925)
}
