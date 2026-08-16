//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2351/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351(t2110: f64, t24514: f64, t26070: f64, t26073: f64, t26076: f64, t27303: f64, t27365: f64, t27961: f64, t27982: f64, t7256: f64, t7259: f64, t7435: f64, t7975: f64, t85480: f64, t85536: f64, t96403: f64, t96559: f64, t96562: f64) -> f64 {
    let t104942 = t96559 * t2110 / 3.0_f64 + t96562 * t2110 / 3.0_f64 + t27982 * t7256 / 3.0_f64 + t27982 * t7259 / 3.0_f64 - 5.0_f64 * t85536 * t27961 - 5.0_f64 * t85480 * t27961 - 5.0_f64 * t24514 * t96403 + 2.0_f64 / 3.0_f64 * t26070 * t7975 + 2.0_f64 / 3.0_f64 * t26073 * t7975 + 2.0_f64 / 3.0_f64 * t26076 * t7975 + 2.0_f64 / 3.0_f64 * t7435 * t27365 + 2.0_f64 / 3.0_f64 * t7435 * t27303;
    t104942
}
