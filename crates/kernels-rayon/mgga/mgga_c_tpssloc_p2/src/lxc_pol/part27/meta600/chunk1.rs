//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2067/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2067(t1914: f64, t40772: f64, t23547: f64, t381: f64, t23310: f64, t23384: f64, t23460: f64, t6686: f64) -> (f64, f64, f64, f64) {
    let t82312 = t1914 * t40772;
    let t82357 = t23547 * t381;
    let t82380 = t23384 * t23310;
    let t82382 = t23460 * t6686;
    (t82312, t82357, t82380, t82382)
}
