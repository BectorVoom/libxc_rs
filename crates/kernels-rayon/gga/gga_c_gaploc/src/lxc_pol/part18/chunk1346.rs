//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1346/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1346(t10795: f64, t747: f64, t10301: f64, t4349: f64, t605: f64, t10802: f64, t14537: f64, t1383: f64, t17293: f64, t3366: f64, t17571: f64, t3411: f64) -> (f64, f64, f64, f64, f64) {
    let t34013 = t10795 * t747;
    let t34018 = 12.0_f64 * t4349 * t10301 * t605;
    let t34020 = 12.0_f64 * t14537 * t10802;
    let t34023 = 24.0_f64 * t17293 * t3366 * t1383;
    let t34025 = 0.69017266717057349418e1_f64 * t17571 * t3411;
    (t34013, t34018, t34020, t34023, t34025)
}
