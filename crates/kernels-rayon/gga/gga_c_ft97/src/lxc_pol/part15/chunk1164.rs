//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1164/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1164(t245: f64, t1178: f64, t20044: f64, t21: f64, t21780: f64, t267: f64, t4431: f64, t5: f64, t5186: f64, t85501: f64, t89629: f64, t89749: f64, t920: f64) -> f64 {
    let t246 = 10000000.0_f64 <= t245;
    let t89765 = piecewise3(t246, 0.0_f64, t5 * (t89629 + t89749) * t21 / 4.0_f64 + t5 * t21780 * t920 + 3.0_f64 / 2.0_f64 * t5 * t5186 * t4431 + t5 * t1178 * t20044 + t5 * t267 * t85501 / 4.0_f64);
    t89765
}
