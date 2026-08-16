//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3771/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3771(t1214: f64, t20950: f64, t12916: f64, t21165: f64, t3718: f64, t12809: f64, t20796: f64, t13045: f64, t5284: f64, t1248: f64, t1121: f64, t12855: f64, t17170: f64, t17396: f64, t17605: f64, t17690: f64, t17709: f64, t17710: f64, t17736: f64, t17744: f64, t20978: f64, t21037: f64, t3611: f64, t3626: f64, t3720: f64, t44484: f64, t44952: f64, t471: f64, t5245: f64, t5297: f64, t5331: f64, t5332: f64, t56861: f64, t59419: f64, t59423: f64, t59426: f64, t71480: f64) -> (f64, f64, f64) {
    let t72050 = t20950 * t1214;
    let t72064 = t3718 * t12916 * t21165;
    let t72071 = t12809 * t12916 * t20796;
    let t72086 = t13045 * t5284;
    let t72087 = t72086 * t1248;
    let t72092 = -0.17149607247227894789e-2_f64 * t12855 * t3720 * t5332 * t72050 - 0.67751534803863288053e-3_f64 * t59419 - 0.85748036236139473944e-3_f64 * t44484 * t20978 - 0.42874018118069736972e-3_f64 * t5331 * t3720 * t5332 * t471 * t17170 - 0.57165357490759649296e-3_f64 * t72064 - 0.42874018118069736972e-3_f64 * t44952 * t3720 * t71480 * t3611 + 0.28582678745379824648e-3_f64 * t72071 + 0.11433071498151929859e-2_f64 * t56861 * t21037 - 0.11433071498151929859e-2_f64 * t17736 * t3626 * t5245 * t1121 * t5297 + 0.22866142996303859718e-2_f64 * t17396 * t17744 + 0.15244095330869239812e-2_f64 * t59423 - 0.1270341277572436651e-3_f64 * t59426 - 0.2540682555144873302e-2_f64 * t17605 * t17690 + 0.51448821741683684367e-2_f64 * t17709 * t3720 * t17710 * t72087;
    (t72050, t72087, t72092)
}
