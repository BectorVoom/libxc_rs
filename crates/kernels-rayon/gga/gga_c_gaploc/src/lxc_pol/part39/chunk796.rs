//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 796/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk796(t12661: f64, t13050: f64, t13054: f64, t13057: f64, t13060: f64, t13061: f64, t13062: f64, t13855: f64, t13859: f64, t13863: f64, t13867: f64, t13874: f64, t13878: f64, t13882: f64, t13886: f64) -> f64 {
    let t13887 = t13855 - t13050 - 0.76685851907841499354e0_f64 * t12661 + t13054 - t13057 - t13060 - 0.46011511144704899612e1_f64 * t13859 + 0.11502877786176224903e2_f64 * t13863 - 0.69017266717057349418e1_f64 * t13867 + t13061 - t13062 - t13874 + t13878 + t13882 - t13886;
    t13887
}
