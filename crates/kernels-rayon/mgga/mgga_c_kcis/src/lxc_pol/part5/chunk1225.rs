//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1225/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1225(t6406: f64, t9634: f64, t969: f64, t3577: f64, t6804: f64, t1219: f64, t5233: f64, t5237: f64, t10865: f64, t6788: f64, t10862: f64, t10884: f64, t10898: f64, t15369: f64, t15460: f64, t20475: f64, t20479: f64, t20486: f64, t20489: f64, t20492: f64, t20495: f64, t20498: f64, t3550: f64, t3575: f64, t3585: f64, t3592: f64, t5216: f64, t5238: f64) -> f64 {
    let t20501 = t6406 * t9634;
    let t20502 = t20501 * t969;
    let t20505 = t6804 * t3577;
    let t20506 = t20505 * t1219;
    let t20509 = t5237 * t5233;
    let t20512 = t6788 * t10865;
    let t20513 = t20512 * t1219;
    let t20516 = -0.11696446794910408142e1_f64 * t3585 * t20475 + 0.17315755899375863299e2_f64 * t3592 * t20479 - 4.0_f64 * t15460 * t5216 + 0.64329366355741395948e2_f64 * t15369 * t5238 + 6.0_f64 * t3575 * t20486 - 4.0_f64 * t3550 * t20489 - 0.19298809906722418785e3_f64 * t10898 * t20492 - 2.0_f64 * t3550 * t20495 + 0.34631511798751726598e2_f64 * t3592 * t20498 + 0.1025389702100779493e4_f64 * t10884 * t20502 + 0.32164683177870697974e2_f64 * t3575 * t20506 + 0.64329366355741395948e2_f64 * t3575 * t20509 + 0.20691336878655965246e4_f64 * t10862 * t20513;
    t20516
}
