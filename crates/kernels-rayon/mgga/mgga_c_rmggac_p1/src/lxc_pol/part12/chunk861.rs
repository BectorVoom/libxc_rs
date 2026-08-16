//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 861/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk861(t1175: f64, t236: f64, t3352: f64, t551: f64, t8517: f64, t1971: f64, t3351: f64, t3924: f64, t5223: f64, t623: f64, t7262: f64, t7265: f64) -> (f64, f64, f64) {
    let t39009 = t8517 * t3352 * t236 * t551 * t1175;
    let t39016 = t3351 * t1971 * t3924 * t5223;
    let t39020 = t623 * t7262;
    let t39021 = t39020 * t7265;
    (t39009, t39016, t39021)
}
