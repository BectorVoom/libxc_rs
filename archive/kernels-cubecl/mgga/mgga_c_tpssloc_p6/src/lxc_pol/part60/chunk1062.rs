//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1062/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1062<F: Float>(t130302: F, t130326: F, t130342: F, t130354: F, t130455: F, t130463: F, t130472: F, t130492: F, t105105: F, t117687: F, t124676: F, t127608: F, t127627: F, t127646: F, t127701: F, t127706: F, t127708: F, t127714: F, t128984: F, t128988: F, t129282: F, t1458: F, t2039: F, t24972: F, t27921: F, t29425: F, t32406: F, t5456: F, t5493: F, t577: F, t7801: F, t7956: F, t96334: F) -> (F, F) {
    let t130495 = t130302 + t130326 + t130342 + t130354 + t130455 + t130463 + t130472 + t130492;
    let t130498 = t127701 + F::cast_from(27.0_f64) * t124676 * t1458 + F::cast_from(0.135e2_f64) * t105105 * t2039 + F::cast_from(27.0_f64) * t129282 * t2039 + F::cast_from(27.0_f64) * t24972 * t29425 + t127608 + t127706 + t127708 + F::cast_from(0.135e2_f64) * t32406 * t5493 + t127714 + t127627 + F::cast_from(54.0_f64) * t96334 * t7956 + F::cast_from(27.0_f64) * t117687 * t5456 + F::cast_from(27.0_f64) * t27921 * t7801 + F::cast_from(0.45e1_f64) * t130495 * t577 + t127646 + t128984 + t128988;
    (t130495, t130498)
}
