//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2179/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2179(t1307: f64, t16094: f64, t56467: f64, t686: f64, t19767: f64, t40409: f64, t19771: f64, t3726: f64, t12199: f64, t19775: f64, t19783: f64, t54670: f64) -> (f64, f64, f64, f64, f64) {
    let t56514 = t16094 * t686 * t56467 * t1307;
    let t56535 = t40409 * t19767;
    let t56537 = t3726 * t19771;
    let t56539 = t12199 * t19775;
    let t56548 = t54670 * t19783;
    (t56514, t56535, t56537, t56539, t56548)
}
