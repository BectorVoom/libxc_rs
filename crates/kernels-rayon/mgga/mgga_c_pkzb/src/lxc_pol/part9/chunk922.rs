//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 922/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk922(t7024: f64, t83: f64, t5077: f64, t5091: f64, t5130: f64, t5139: f64, t5141: f64, t5148: f64, t7013: f64, t7015: f64, t7017: f64, t7018: f64, t7019: f64, t7020: f64, t7021: f64, t7022: f64, t7023: f64) -> (f64, f64) {
    let t7025 = t83 * t7024;
    let t7026 = t5077 - t7013 + t7015 - t7017 - t7018 - t7019 + t5091 - t5130 - t7020 - t7021 - t5139 - t5141 + t7022 - t5148 + t7023 + t7025;
    (t7025, t7026)
}
