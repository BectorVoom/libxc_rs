//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1274/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1274(t1240: f64, t8087: f64, t7284: f64, t8054: f64, t1170: f64, t2121: f64, t34237: f64, t24574: f64, t34251: f64, t24826: f64, t34292: f64, t118111: f64, t1209: f64, t1244: f64, t1246: f64, t1734: f64, t2144: f64, t24812: f64, t24813: f64, t24833: f64, t27470: f64, t27491: f64, t27497: f64, t27507: f64, t27536: f64, t27549: f64, t27550: f64, t27724: f64, t3242: f64, t32451: f64, t32465: f64, t32466: f64, t34291: f64, t3502: f64, t3961: f64, t7373: f64, t7375: f64, t7376: f64) -> (f64, f64, f64, f64, f64) {
    let t125295 = t1240 * t8087;
    let t125306 = t7284 * t8054;
    let t125311 = t2121 * t1170 * t34237;
    let t125313 = t24574 * t34251;
    let t125351 = t24826 * t34292;
    let t125358 = 0.73108180748810063844e-2_f64 * t27549 * t27550 * t2144 * t3242 * t3961 - 0.16449340668482264365e-1_f64 * t7373 * t27536 * t32465 + 0.54831135561607547883e-2_f64 * t118111 - 0.16449340668482264365e-1_f64 * t7373 * t24833 * t34291 + 0.3289868133696452873e-1_f64 * t24812 * t24813 * t3502 * t2144 * t27491 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t27724 * t7376 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t27470 * t7376 - 0.43864908449286038307e-1_f64 * t27507 * t32466 + t1244 * t32451 * t1734 * t1246 + 0.54831135561607547883e-2_f64 * t125351 - 0.16449340668482264365e-1_f64 * t24812 * t24813 * t1209 * t2144 * t27497;
    (t125295, t125306, t125311, t125313, t125358)
}
