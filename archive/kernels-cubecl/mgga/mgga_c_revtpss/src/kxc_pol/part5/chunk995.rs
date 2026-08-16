//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 995/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk995<F: Float>(t2470: F, t2804: F, t874: F, t875: F, t9288: F, t2718: F, t860: F, t243: F, t816: F, t9707: F, t813: F, t2689: F, t2694: F) -> (F, F, F, F, F) {
    let t10647 = t874 * t2804 * t2470;
    let t10651 = F::cast_from(0.30356481678079769392e-1_f64) * t874 * t875 * t9288;
    let t10661 = t2718 * t860;
    let t10671 = t9707 * t243 * t816;
    let t10673 = F::cast_from(0.12846167376791569079e-2_f64) * t813 * t10671;
    let t10678 = t2689 * t2694;
    (t10647, t10651, t10661, t10673, t10678)
}
