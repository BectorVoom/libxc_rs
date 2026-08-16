//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2304/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2304<F: Float>(t1011: F, t5866: F, t1948: F, t7577: F, t1023: F, t23601: F, t23657: F, t25429: F, t25484: F, t25491: F, t25502: F, t25523: F, t25540: F, t25544: F, t25660: F, t25722: F, t28621: F, t28651: F, t4594: F, t6797: F, t7610: F, t83245: F, t83265: F, t89002: F, t89033: F, t89049: F, t89057: F, t89395: F) -> (F, F) {
    let t100075 = t5866 * t1011;
    let t100087 = t7577 * t1948;
    let t100103 = -t89049 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t25523 * t25502 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t23657 * t28621 + F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t25484 * t100075 * t4594 - F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t25491 * t100075 * t1023 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t89002 * t7610 - F::cast_from(0.73108180748810063845e-2_f64) * t25429 * t100087 * t25722 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t25523 * t25540 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t25523 * t25544 - F::cast_from(0.54831135561607547884e-2_f64) * t83245 * t83265 * t28651 * t25660 - F::cast_from(0.54831135561607547884e-2_f64) * t89033 * t89395 + t89057;
    (t100087, t100103)
}
