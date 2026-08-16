//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1274/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1274(t1457: f64, t2103: f64, t32210: f64, t10817: f64, t7736: f64, t1853: f64, t191: f64, t3039: f64, t7635: f64, t21491: f64, t8793: f64, t10915: f64, t22242: f64, t32514: f64) -> (f64, f64, f64, f64, f64) {
    let t33004 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t32210;
    let t33009 = 0.25025342966295298669e1_f64 * t10817 * t7736;
    let t33013 = 0.71500979903700853338e0_f64 * t7635 * t3039 * t191 * t1853;
    let t33018 = 0.50050685932590597338e1_f64 * t8793 * t1457 * t21491;
    let t33021 = 0.42900587942220512002e1_f64 * t22242 * t10915 * t32514;
    (t33004, t33009, t33013, t33018, t33021)
}
