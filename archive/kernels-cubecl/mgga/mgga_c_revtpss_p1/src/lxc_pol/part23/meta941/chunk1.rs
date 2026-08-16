//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3092/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092<F: Float>(t16784: F, t6548: F, t24494: F, t3531: F, t1196: F, t5181: F, t6555: F, t20896: F, t5192: F, t81352: F, t81558: F, t81560: F, t81562: F, t81566: F, t81570: F, t81573: F) -> (F, F, F, F, F) {
    let t81575 = F::cast_from(0.35089341735807877242e1_f64) * t16784 * t6548;
    let t81577 = F::cast_from(0.10389515463408878255e3_f64) * t3531 * t24494;
    let t81580 = F::cast_from(0.10526802520742363173e2_f64) * t1196 * t6555 * t5181;
    let t81582 = F::cast_from(0.51947577317044391276e2_f64) * t5192 * t20896;
    let t81583 = t81352 + t81558 + t81560 - t81562 + t81566 + t81570 + t81573 + t81575 + t81577 - t81580 - t81582;
    (t81575, t81577, t81580, t81582, t81583)
}
