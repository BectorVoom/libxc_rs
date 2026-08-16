//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2346/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346(t5: f64, t95996: f64, t96021: f64, t96050: f64, t96077: f64, t96105: f64, t96133: f64, t96180: f64, t96209: f64, t112: f64, t671: f64, t7263: f64, t12813: f64, t1459: f64, t1849: f64, t2165: f64, t2314: f64, t24932: f64, t24939: f64, t27293: f64, t3929: f64, t4037: f64, t510: f64, t652: f64, t8107: f64, t91666: f64, t91671: f64, t91673: f64, t91674: f64, t91678: f64, t91681: f64, t91684: f64, t91690: f64, t91694: f64, t91698: f64, t91704: f64, t91706: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t96213 = piecewise3(t8, 0.0_f64, t95996 + t96021 + t96050 + t96077 + t96105 + t96133 + t96180 + t96209);
    let t96214 = t96213 * t112;
    let t96222 = t7263 * t671;
    let t96228 = -2.0_f64 * t12813 * t2165 * t652 - 4.0_f64 * t1459 * t96222 + t1849 * t24939 - 4.0_f64 * t2314 * t27293 - 4.0_f64 * t24932 * t4037 + t3929 * t8107 - t510 * t96214 + t91666 + t91671 - t91673 - t91674 + t91678 + t91681 - t91684 - t91690 - t91694 - t91698 - t91704 - t91706;
    (t96214, t96222, t96228)
}
