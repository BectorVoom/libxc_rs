//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2216/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2216(t2717: f64, t5636: f64, t22986: f64, t23270: f64, t776: f64, t225: f64, t28437: f64, t258: f64, t5544: f64, t25038: f64, t1888: f64, t5657: f64, t865: f64) -> (f64, f64, f64, f64) {
    let t98161 = t2717 * t5636;
    let t98164 = t22986 * t23270 * t98161 * t776;
    let t98166 = t28437 * t225;
    let t98169 = t258 * t5544;
    let t98172 = t25038 * t23270 * t98169 * t776;
    let t98181 = t1888 * t23270 * t2717 * t5657 * t865;
    (t98164, t98166, t98172, t98181)
}
