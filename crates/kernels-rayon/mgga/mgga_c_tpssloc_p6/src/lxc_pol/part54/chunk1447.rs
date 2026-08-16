//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1447/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1447(t24994: f64, t8606: f64, t24996: f64, t2075: f64, t26135: f64, t652: f64, t2314: f64, t33620: f64, t4034: f64, t1458: f64, t31518: f64, t1873: f64, t92090: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122698 = t8606 * t24994;
    let t122700 = 6.0_f64 * t122698 * t24996;
    let t122706 = 2.0_f64 * t652 * t2075 * t26135;
    let t122708 = 2.0_f64 * t2314 * t33620;
    let t122710 = 2.0_f64 * t4034 * t33620;
    let t122713 = 2.0_f64 * t652 * t31518 * t1458;
    let t122718 = t92090 * t1873;
    (t122700, t122706, t122708, t122710, t122713, t122718)
}
