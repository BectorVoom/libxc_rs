//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1357/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1357(t16937: f64, t29283: f64, t27369: f64, t1385: f64, t27356: f64, t5709: f64, t6284: f64, t12147: f64, t29313: f64, t7908: f64, t28549: f64, t94228: f64, t98240: f64) -> (f64, f64, f64, f64, f64) {
    let t103239 = t16937 * t29283;
    let t103240 = t27369 * t103239;
    let t103251 = t5709 * t27356 * t6284 * t1385;
    let t103255 = t7908 * t12147 * t29313;
    let t103258 = t94228 * t98240 * t28549;
    (t103239, t103240, t103251, t103255, t103258)
}
