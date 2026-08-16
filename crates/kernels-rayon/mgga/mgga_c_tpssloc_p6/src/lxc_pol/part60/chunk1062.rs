//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1062/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1062(t130302: f64, t130326: f64, t130342: f64, t130354: f64, t130455: f64, t130463: f64, t130472: f64, t130492: f64, t105105: f64, t117687: f64, t124676: f64, t127608: f64, t127627: f64, t127646: f64, t127701: f64, t127706: f64, t127708: f64, t127714: f64, t128984: f64, t128988: f64, t129282: f64, t1458: f64, t2039: f64, t24972: f64, t27921: f64, t29425: f64, t32406: f64, t5456: f64, t5493: f64, t577: f64, t7801: f64, t7956: f64, t96334: f64) -> (f64, f64) {
    let t130495 = t130302 + t130326 + t130342 + t130354 + t130455 + t130463 + t130472 + t130492;
    let t130498 = t127701 + 27.0_f64 * t124676 * t1458 + 0.135e2_f64 * t105105 * t2039 + 27.0_f64 * t129282 * t2039 + 27.0_f64 * t24972 * t29425 + t127608 + t127706 + t127708 + 0.135e2_f64 * t32406 * t5493 + t127714 + t127627 + 54.0_f64 * t96334 * t7956 + 27.0_f64 * t117687 * t5456 + 27.0_f64 * t27921 * t7801 + 0.45e1_f64 * t130495 * t577 + t127646 + t128984 + t128988;
    (t130495, t130498)
}
