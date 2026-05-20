//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1613/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613<F: Float>(t87529: F, t87541: F, t5962: F, t5966: F, t124: F, t1544: F, t1559: F, t23266: F, t2730: F, t2745: F, t2747: F, t40507: F, t40607: F, t40611: F, t40868: F, t50436: F, t50611: F, t61677: F, t61699: F, t61797: F, t61833: F, t76279: F, t76500: F, t76502: F, t76572: F, t799: F, t800: F) -> (F, F, F, F) {
    let t87543 = t87529 / F::new(2.0) + t87541 / F::new(2.0);
    let t87548 = t5962 * t5962;
    let t87553 = t5966 * t5966;
    let t87562 = F::cast_from(0.68026775414003982664e0_f64) * t61677 + F::cast_from(0.27210710165601593065e0_f64) * t61699 + t2730 * t800 * t23266 * t1544 / F::new(4.0) + F::cast_from(0.12004725073059526352e-1_f64) * t76500 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t76279 * t1559 + F::cast_from(0.96037800584476210818e-1_f64) * t76502 - F::cast_from(0.80328230880474379775e-6_f64) * t50436 + t40507 - t799 * t800 * t124 * t87543 / F::new(48.0) + F::new(3.0) / F::new(16.0) * t2730 * t800 * t124 * t87548 + F::new(5.0) / F::new(4.0) * t40868 * t800 * t124 * t87553 + F::cast_from(0.15246000842785598467e-4_f64) * t61797 + F::cast_from(0.32528867398167352889e-3_f64) * t50611 - F::cast_from(0.30492001685571196936e-3_f64) * t61833 - F::cast_from(0.17149607247227894789e-3_f64) * t76572 + t40607 - t40611;
    (t87543, t87548, t87553, t87562)
}
