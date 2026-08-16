//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1177/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1177(t10525: f64, t10526: f64, t47803: f64, t6717: f64, t6914: f64, t12079: f64, t2389: f64, t12092: f64, t2482: f64, t9267: f64, t40009: f64, t41697: f64, t41699: f64, t41700: f64, t41703: f64, t41706: f64, t41712: f64, t41713: f64) -> f64 {
    let t47860 = t10525 * t10526 * t47803;
    let t47864 = t6914 * t6717 * t47803;
    let t47866 = t12079 * t2389;
    let t47869 = t9267 * t12092 * t2482;
    let t47871 = 0.63904876589867916128e-1_f64 * t40009;
    let t47872 = -t41697 + t41699 - 0.21450293971110256001e1_f64 * t47860 - t41700 - 0.46011511144704899612e1_f64 * t41703 - 0.62115540045351614476e2_f64 * t47864 - 0.29792074959875355558e-1_f64 * t47866 - t41706 - t41712 + 0.9585731488480187419e0_f64 * t47869 + t41713 - t47871;
    t47872
}
