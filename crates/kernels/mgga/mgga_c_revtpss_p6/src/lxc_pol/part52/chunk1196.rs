//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1196/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1196<F: Float>(t1913: F, t8734: F, t34369: F, t571: F, t102005: F, t28196: F, t34297: F, t670: F, t8626: F, t122570: F, t125496: F, t1519: F, t2055: F, t2322: F, t27830: F, t28929: F, t32389: F, t32621: F, t33578: F, t33580: F, t33583: F, t34290: F, t4254: F, t4257: F, t4293: F, t651: F, t7732: F) -> (F, F, F, F) {
    let t127515 = t1913 * t8734;
    let t127516 = t571 * t34369;
    let t127532 = F::cast_from(2.0_f64) * t28196 * t102005 * t34297;
    let t127535 = t8626 * t670;
    let t127540 = -F::cast_from(2.0_f64) * t2055 * t27830 * t651 - F::cast_from(2.0_f64) * t122570 * t1519 + F::cast_from(6.0_f64) * t125496 * t28929 - F::cast_from(2.0_f64) * t127535 * t1519 - F::cast_from(2.0_f64) * t2322 * t34290 - F::cast_from(2.0_f64) * t32389 * t4257 - F::cast_from(2.0_f64) * t32389 * t4293 - F::cast_from(2.0_f64) * t32621 * t7732 - F::cast_from(2.0_f64) * t34290 * t4254 + t127532 - t33578 - t33580 - t33583;
    (t127515, t127516, t127535, t127540)
}
