//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 640/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk640(t2364: f64, t6697: f64, t1800: f64, t1799: f64, t1801: f64, t8514: f64, t5203: f64, t8780: f64, t1869: f64, t2441: f64, t6965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8866 = t6697 * t2364;
    let t8867 = t1800 * t8866;
    let t8868 = t1799 * t8867;
    let t8870 = t1801 * t8514;
    let t8871 = t1800 * t8870;
    let t8872 = t1799 * t8871;
    let t8874 = t5203 * t8780;
    let t8875 = t1800 * t8874;
    let t8876 = t1869 * t8875;
    let t8878 = t6965 * t2441;
    let t8879 = t1800 * t8878;
    let t8880 = t1869 * t8879;
    (t8866, t8867, t8868, t8870, t8871, t8872, t8874, t8875, t8876, t8878, t8879, t8880)
}
