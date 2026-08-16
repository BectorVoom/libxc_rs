//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2304/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2304(t1011: f64, t5866: f64, t1948: f64, t7577: f64, t1023: f64, t23601: f64, t23657: f64, t25429: f64, t25484: f64, t25491: f64, t25502: f64, t25523: f64, t25540: f64, t25544: f64, t25660: f64, t25722: f64, t28621: f64, t28651: f64, t4594: f64, t6797: f64, t7610: f64, t83245: f64, t83265: f64, t89002: f64, t89033: f64, t89049: f64, t89057: f64, t89395: f64) -> (f64, f64) {
    let t100075 = t5866 * t1011;
    let t100087 = t7577 * t1948;
    let t100103 = -t89049 - 0.16449340668482264365e-1_f64 * t6797 * t25523 * t25502 - 0.82246703342411321825e-2_f64 * t6797 * t23657 * t28621 + 0.16449340668482264365e-1_f64 * t23601 * t25484 * t100075 * t4594 - 0.82246703342411321825e-2_f64 * t23601 * t25491 * t100075 * t1023 - 0.16449340668482264365e-1_f64 * t6797 * t89002 * t7610 - 0.73108180748810063845e-2_f64 * t25429 * t100087 * t25722 - 0.16449340668482264365e-1_f64 * t6797 * t25523 * t25540 - 0.16449340668482264365e-1_f64 * t6797 * t25523 * t25544 - 0.54831135561607547884e-2_f64 * t83245 * t83265 * t28651 * t25660 - 0.54831135561607547884e-2_f64 * t89033 * t89395 + t89057;
    (t100087, t100103)
}
