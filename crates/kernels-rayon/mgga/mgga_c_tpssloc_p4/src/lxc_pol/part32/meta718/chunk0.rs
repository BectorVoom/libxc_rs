//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2283/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283(t100674: f64, t100716: f64, t100763: f64, t100803: f64, t24987: f64, t7754: f64, t1983: f64, t2019: f64, t57806: f64, t25971: f64, t91655: f64, t26161: f64, t26162: f64, t75210: f64) -> (f64, f64, f64, f64, f64) {
    let t100805 = t100674 + t100716 + t100763 + t100803;
    let t100828 = 2.0_f64 * t24987 * t7754;
    let t100833 = t1983 * t2019 * t57806;
    let t100835 = 6.0_f64 * t91655 * t25971;
    let t100838 = 2.0_f64 * t26161 * t26162 * t75210;
    (t100805, t100828, t100833, t100835, t100838)
}
