//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 901/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk901(t8953: f64, t967: f64, t2719: f64, t956: f64, t2713: f64, t2716: f64, t941: f64, t2751: f64, t774: f64, t348: f64, t2738: f64, t983: f64) -> (f64, f64, f64, f64, f64) {
    let t8954 = t967 * t8953;
    let t8970 = t956 * t2719;
    let t8972 = t2713 * t2716 * t8970;
    let t8976 = t2713 * t941 * t8970;
    let t8983 = t774 * t2751;
    let t8987 = t348 * t956;
    let t8989 = t983 * t8987 * t2738;
    (t8954, t8972, t8976, t8983, t8989)
}
