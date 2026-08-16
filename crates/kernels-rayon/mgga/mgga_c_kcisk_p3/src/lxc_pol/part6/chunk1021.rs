//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1021/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1021(t338: f64, t30738: f64, t1320: f64, t1310: f64, t2168: f64, t8048: f64, t13831: f64, t2075: f64, t8054: f64, t3937: f64, t7736: f64, t13504: f64, t1309: f64, t13873: f64, t20169: f64, t20185: f64, t2170: f64, t26008: f64, t26430: f64, t26471: f64, t26485: f64, t26490: f64, t30536: f64, t30540: f64, t30544: f64, t30548: f64, t3935: f64, t6157: f64, t8050: f64, t8056: f64) -> f64 {
    let t400 = 0.0_f64 < t338;
    let t30740 = piecewise3(t400, t30738, -t30738);
    let t30741 = t1320 * t30740;
    let t30742 = t1310 * t30741;
    let t30747 = t8048 * t2168;
    let t30748 = t13831 * t30747;
    let t30749 = t1310 * t30748;
    let t30758 = t2075 * t8054;
    let t30759 = t3937 * t30758;
    let t30762 = t7736 * t2168;
    let t30763 = t13504 * t30762;
    let t30769 = 0.53972366148531951639e-1_f64 * t26430 + 0.71963154864709268853e-1_f64 * t3935 * t30536 + 0.10794473229706390328e0_f64 * t1309 * t30540 - 0.1439263097294185377e0_f64 * t1309 * t30544 + 0.55971342672551653552e-1_f64 * t1309 * t30548 - 0.5397236614853195164e-1_f64 * t1309 * t30742 + 0.32383419689119170984e0_f64 * t6157 * t8050 - 0.32383419689119170984e0_f64 * t1309 * t30749 - 0.16191709844559585492e0_f64 * t26008 * t2170 - 0.35981577432354634426e-1_f64 * t20169 + 0.35981577432354634426e-1_f64 * t20185 - 0.16191709844559585492e0_f64 * t6157 * t8056 - 0.53972366148531951639e-1_f64 * t3935 * t30759 - 0.71963154864709268852e-1_f64 * t3935 * t30763 + t13873 - 0.53972366148531951639e-1_f64 * t26471 - 0.10794473229706390328e0_f64 * t26485 + 0.10794473229706390328e0_f64 * t26490;
    t30769
}
