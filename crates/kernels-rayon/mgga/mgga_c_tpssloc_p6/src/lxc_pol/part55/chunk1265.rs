//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1265/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1265(t31832: f64, t7754: f64, t8689: f64, t8944: f64, t26164: f64, t24994: f64, t24996: f64, t26149: f64, t8690: f64, t12725: f64, t8675: f64, t33690: f64, t6535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123193 = t31832 * t7754;
    let t123194 = t8689 * t8944;
    let t123195 = t123194 * t26164;
    let t123198 = t8689 * t24994;
    let t123199 = t123198 * t24996;
    let t123205 = t8690 * t26149;
    let t123206 = t12725 * t8675;
    let t123211 = t33690 * t6535;
    (t123193, t123195, t123199, t123205, t123206, t123211)
}
