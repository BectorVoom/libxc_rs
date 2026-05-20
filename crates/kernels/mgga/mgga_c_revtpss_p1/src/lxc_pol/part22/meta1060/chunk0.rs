//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3771/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3771<F: Float>(t1214: F, t20950: F, t12916: F, t21165: F, t3718: F, t12809: F, t20796: F, t13045: F, t5284: F, t1248: F, t1121: F, t12855: F, t17170: F, t17396: F, t17605: F, t17690: F, t17709: F, t17710: F, t17736: F, t17744: F, t20978: F, t21037: F, t3611: F, t3626: F, t3720: F, t44484: F, t44952: F, t471: F, t5245: F, t5297: F, t5331: F, t5332: F, t56861: F, t59419: F, t59423: F, t59426: F, t71480: F) -> (F, F, F) {
    let t72050 = t20950 * t1214;
    let t72064 = t3718 * t12916 * t21165;
    let t72071 = t12809 * t12916 * t20796;
    let t72086 = t13045 * t5284;
    let t72087 = t72086 * t1248;
    let t72092 = -F::cast_from(0.17149607247227894789e-2_f64) * t12855 * t3720 * t5332 * t72050 - F::cast_from(0.67751534803863288053e-3_f64) * t59419 - F::cast_from(0.85748036236139473944e-3_f64) * t44484 * t20978 - F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t3720 * t5332 * t471 * t17170 - F::cast_from(0.57165357490759649296e-3_f64) * t72064 - F::cast_from(0.42874018118069736972e-3_f64) * t44952 * t3720 * t71480 * t3611 + F::cast_from(0.28582678745379824648e-3_f64) * t72071 + F::cast_from(0.11433071498151929859e-2_f64) * t56861 * t21037 - F::cast_from(0.11433071498151929859e-2_f64) * t17736 * t3626 * t5245 * t1121 * t5297 + F::cast_from(0.22866142996303859718e-2_f64) * t17396 * t17744 + F::cast_from(0.15244095330869239812e-2_f64) * t59423 - F::cast_from(0.1270341277572436651e-3_f64) * t59426 - F::cast_from(0.2540682555144873302e-2_f64) * t17605 * t17690 + F::cast_from(0.51448821741683684367e-2_f64) * t17709 * t3720 * t17710 * t72087;
    (t72050, t72087, t72092)
}
