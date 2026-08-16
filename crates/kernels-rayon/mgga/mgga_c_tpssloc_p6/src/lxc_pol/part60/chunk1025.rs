//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1025/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1025(t2018: f64, t26161: f64, t6324: f64, t92169: f64, t33363: f64, t7688: f64, t28017: f64, t89: f64, t2040: f64, t33214: f64, t7796: f64, t28030: f64, t8533: f64) -> (f64, f64, f64, f64, f64) {
    let t128498 = 6.0_f64 * t26161 * t92169 * t2018 * t6324;
    let t128502 = 6.0_f64 * t33363 * t7688;
    let t128507 = t89 * t28017;
    let t128509 = 2.0_f64 * t128507 * t2040;
    let t128511 = 4.0_f64 * t33214 * t7796;
    let t128513 = 2.0_f64 * t28030 * t8533;
    (t128498, t128502, t128509, t128511, t128513)
}
