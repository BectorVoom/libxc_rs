//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1382/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1382(t26142: f64, t7266: f64, t25985: f64, t8690: f64, t120064: f64, t120067: f64, t120069: f64, t120072: f64, t120075: f64, t120078: f64, t120079: f64, t120083: f64, t120085: f64, t120086: f64, t120088: f64) -> f64 {
    let t123168 = t7266 * t26142;
    let t123173 = t8690 * t25985;
    let t123175 = -2.0_f64 * t120064 - t120067 - t120069 + t120072 - t120075 + t120078 - 2.0_f64 * t123168 + 3.0_f64 * t120079 - t120083 + t120085 + 3.0_f64 * t120086 + 3.0_f64 * t120088 + 3.0_f64 * t123173;
    t123175
}
