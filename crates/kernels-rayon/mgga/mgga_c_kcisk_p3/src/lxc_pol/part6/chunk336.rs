//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 336/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk336(t222: f64, t2059: f64, t224: f64, zeta_threshold: f64) -> (f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t2062 = piecewise3(t223, 0.0_f64, 4.0_f64 / 3.0_f64 * t224 * t2059);
    let t2063 = -t2059;
    (t2062, t2063)
}
