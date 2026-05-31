//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 571/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk571<F: Float>(t2710: F, t2713: F, t826: F, t232: F, t821: F, t235: F) -> (F, F, F) {
    let t2716 = F::cast_from(0.45178982497454656791e-5_f64) * t2710 * t2713 * t826;
    let t2718 = F::cast_from(1.0_f64) / t821 / t232;
    let t2719 = t2718 * t235;
    (t2716, t2718, t2719)
}
