//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1592/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1592(t43806: f64, t43856: f64, t43936: f64, t43959: f64, t1179: f64, t1188: f64, t1196: f64, t3515: f64, t3520: f64, t3523: f64, t3794: f64, t12555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43961 = t43806 + t43856 + t43936 + t43959;
    let t43965 = 0.5848223622634646207e0_f64 * t1196 * t1179 * t43961 * t1188;
    let t43966 = t3515 * t3515;
    let t43970 = 0.51947577317044391277e2_f64 * t1196 * t3520 * t43966 * t3523;
    let t43971 = t3794 * t3794;
    let t43977 = t12555 * t3515;
    (t43961, t43965, t43966, t43970, t43971, t43977)
}
