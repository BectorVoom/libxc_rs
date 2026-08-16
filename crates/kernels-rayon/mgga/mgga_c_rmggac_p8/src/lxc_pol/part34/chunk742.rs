//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 742/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk742(t1341: f64, t638: f64, t703: f64, t7310: f64, t69701: f64, t69760: f64, t69819: f64, t69832: f64, t69860: f64, t69865: f64, t14567: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71446 = t638 * t7310 * t703 * t1341;
    let t71447 = 0.30487649791575028314e-3_f64 * t71446;
    let t71448 = 0.22800128353348964998e-6_f64 * t69701;
    let t71486 = 0.10986805899793472145e-3_f64 * t69760;
    let t71502 = 0.19516036795685772888e-4_f64 * t69819;
    let t71505 = 0.68400385060046895e-6_f64 * t69832;
    let t71513 = 0.69390353051327192491e-4_f64 * t69860;
    let t71514 = 0.13010691197123848592e-4_f64 * t69865;
    let t71516 = t942 * t14567;
    (t71447, t71448, t71486, t71502, t71505, t71513, t71514, t71516)
}
