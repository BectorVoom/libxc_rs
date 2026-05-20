//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2394/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2394<F: Float>(t41245: F, t41306: F, t2966: F, t302: F, t2969: F, t11506: F, t960: F, t315: F, t41224: F, t11408: F, t941: F, t11465: F) -> (F, F, F, F, F, F, F, F) {
    let t41672 = F::cast_from(0.16979925925925925926e1_f64) * t41245;
    let t41690 = F::cast_from(0.5356037037037037037e1_f64) * t41306;
    let t41738 = t2966 * t2966;
    let t41740 = t302 / t41738;
    let t41741 = t2969 * t2969;
    let t41742 = F::new(1.0) / t41741;
    let t41756 = t960 * t11506;
    let t41759 = t315 * t41224;
    let t41779 = t941 * t11408;
    let t41788 = t960 * t11465;
    (t41672, t41690, t41740, t41742, t41756, t41759, t41779, t41788)
}
