//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 740/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk740(t2227: f64, t235: f64, t7262: f64, t14696: f64, t7491: f64, t1341: f64, t638: f64, t703: f64, t7310: f64, t69760: f64, t69832: f64, t69934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71404 = t235 * t7262 * t2227;
    let t71418 = t7491 * t14696;
    let t71419 = 0.30487649791575028314e-3_f64 * t71418;
    let t71446 = t638 * t7310 * t703 * t1341;
    let t71447 = 0.30487649791575028314e-3_f64 * t71446;
    let t71486 = 0.10986805899793472145e-3_f64 * t69760;
    let t71505 = 0.68400385060046895e-6_f64 * t69832;
    let t71544 = 0.30487649791575028312e-3_f64 * t69934;
    (t71404, t71419, t71447, t71486, t71505, t71544)
}
