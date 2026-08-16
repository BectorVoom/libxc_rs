//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1237/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1237(t24776: f64, t29776: f64, t6256: f64, t7376: f64, t7375: f64, t27516: f64, t8066: f64, t1716: f64, t8077: f64, t1729: f64, t2152: f64, t24589: f64, t24812: f64, t27406: f64, t27507: f64, t27572: f64, t27728: f64, t27737: f64, t29750: f64, t29754: f64, t29759: f64, t29763: f64, t29773: f64, t470: f64, t6168: f64, t7283: f64, t7373: f64, t7999: f64, t8067: f64, t8074: f64, t8078: f64, t8085: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29777 = t24776 * t29776;
    let t29781 = t6256 * t7376;
    let t29782 = t7375 * t29781;
    let t29787 = t27516 * t8066;
    let t29790 = t1716 * t8077;
    let t29793 = 0.16449340668482264365e-1_f64 * t24812 * t29750 - 0.82246703342411321825e-2_f64 * t24812 * t29754 - 0.14621636149762012769e-1_f64 * t27572 - 0.27415567780803773942e-2_f64 * t7283 * t29759 - 0.54831135561607547884e-2_f64 * t7283 * t29763 + 0.14621636149762012769e-1_f64 * t27406 * t8067 - 0.43864908449286038306e-1_f64 * t7999 * t8078 + t6168 * t2152 + 2.0_f64 * t1729 * t8085 + t470 * t29773 - 0.54831135561607547884e-2_f64 * t27728 + 0.36554090374405031923e-2_f64 * t7283 * t29777 + 0.54831135561607547884e-2_f64 * t27737 + 0.16449340668482264365e-1_f64 * t7373 * t29782 - 0.43864908449286038306e-1_f64 * t27507 * t8074 + 0.54831135561607547884e-2_f64 * t24589 * t29787 - 0.16449340668482264365e-1_f64 * t7283 * t29790;
    (t29777, t29781, t29782, t29787, t29790, t29793)
}
