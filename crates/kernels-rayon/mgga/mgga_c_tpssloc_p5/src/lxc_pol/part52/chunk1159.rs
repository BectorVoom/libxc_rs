//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1159/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1159(t1877: f64, t193: f64, t202: f64, t2522: f64, t30752: f64, t30757: f64, t30770: f64, t6665: f64, t6670: f64, t776: f64, t8366: f64, t8370: f64, t868: f64, t870: f64) -> f64 {
    let t30952 = t193 * t202 * t30752 * t870 - t1877 * t30757 * t868 + 2.0_f64 * t1877 * t30770 * t868 - 2.0_f64 * t1877 * t6665 * t6670 + 3.0_f64 * t2522 * t776 * t8366 - 3.0_f64 * t2522 * t776 * t8370;
    t30952
}
