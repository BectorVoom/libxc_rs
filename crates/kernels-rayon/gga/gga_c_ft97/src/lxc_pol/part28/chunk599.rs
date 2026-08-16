//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 599/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk599(t25653: f64, t73: f64, t3056: f64, t373: f64, t35: f64, t5537: f64, t5546: f64, t929: f64, t11121: f64, t1603: f64, t1669: f64, t22574: f64, t22613: f64, t22619: f64, t22634: f64, t22644: f64, t22696: f64, t22736: f64, t22738: f64, t22834: f64, t25626: f64, t25631: f64, t25637: f64, t25640: f64, t25644: f64, t25649: f64, t3019: f64, t5538: f64, t5540: f64, t6427: f64, t6428: f64, t6431: f64) -> (f64, f64, f64) {
    let t25654 = t73 * t25653;
    let t25657 = t373 * t3056;
    let t25658 = t25657 * t35;
    let t25663 = t5537 * t5546 * t929;
    let t25669 = 0.13519760450715832853e-3_f64 * t3019 * t25626 - 0.23254900946437792e-1_f64 * t22834 * t6428 - 0.23254900946437792e-1_f64 * t1603 * t25631 + 0.74233839446572641111e-4_f64 * t22574 - 2.0_f64 * t22696 * t6431 - 2.0_f64 * t1669 * t25637 - 2.0_f64 * t1669 * t25640 + 4.0_f64 * t1669 * t25644 + 0.12768721675925925926e-1_f64 * t22634 - 0.15137014751914110597e-3_f64 * t22644 + 0.44540303667943584666e-3_f64 * t22613 * t73 * t25649 - 0.44540303667943584666e-3_f64 * t22619 * t25654 + 0.25845121844514357744e-4_f64 * t5538 * t5540 * t25658 - 0.60102574844279699039e-6_f64 * t11121 * t25663 + 0.61277550024922479209e-6_f64 * t22736 * t22738 * t6427;
    (t25654, t25658, t25669)
}
