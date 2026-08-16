//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1019/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1019(t2018: f64, t24432: f64, t24995: f64, t6330: f64, t26161: f64, t6324: f64, t92169: f64, t33363: f64, t7688: f64, t28017: f64, t89: f64, t2040: f64) -> (f64, f64, f64, f64) {
    let t128492 = 6.0_f64 * t24995 * t24432 * t2018 * t6330;
    let t128498 = 6.0_f64 * t26161 * t92169 * t2018 * t6324;
    let t128502 = 6.0_f64 * t33363 * t7688;
    let t128507 = t89 * t28017;
    let t128509 = 2.0_f64 * t128507 * t2040;
    (t128492, t128498, t128502, t128509)
}
