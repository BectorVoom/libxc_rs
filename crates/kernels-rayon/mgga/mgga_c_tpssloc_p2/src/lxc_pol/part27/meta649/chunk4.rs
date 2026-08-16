//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2254/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2254(t23788: f64, t59580: f64, t86815: f64, t13196: f64, t25891: f64, t25927: f64, t58009: f64, t10143: f64, t1081: f64, t25374: f64, t4255: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89837 = t23788 * t59580;
    let t89840 = t23788 * t86815;
    let t89843 = t25891 * t13196;
    let t89846 = t25927 * t58009;
    let t89849 = t10143 * t1081;
    let t89850 = t89849 * t25374;
    let t89859 = t870 * t1081 * t4255;
    (t89837, t89840, t89843, t89846, t89850, t89859)
}
