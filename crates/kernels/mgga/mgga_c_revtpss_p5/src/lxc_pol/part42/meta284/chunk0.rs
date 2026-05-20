//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1040/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1040<F: Float>(t10671: F, t813: F, t2689: F, t2694: F, t243: F, t247: F, t9949: F, t237: F, t236: F, t9646: F, t9721: F, t268: F) -> (F, F, F, F, F) {
    let t10673 = F::cast_from(0.12846167376791569079e-2_f64) * t813 * t10671;
    let t10678 = t2689 * t2694;
    let t10685 = t9949 * t243 * t247;
    let t10687 = F::cast_from(0.37792653007779990369e-1_f64) * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    (t10673, t10678, t10687, t10688, t10690)
}
