//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1323/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1323<F: Float>(t10654: F, t2782: F, t2760: F, t822: F, t243: F, t816: F, t9707: F, t813: F, t2394: F, t2476: F, t236: F, t807: F) -> (F, F, F, F, F, F) {
    let t10655 = t2782 * t10654;
    let t10657 = t822 * t2760;
    let t10671 = t9707 * t243 * t816;
    let t10673 = F::cast_from(0.12846167376791569079e-2_f64) * t813 * t10671;
    let t10674 = t2476 * t2394;
    let t10675 = t236 * t10674;
    let t10676 = t807 * t10675;
    (t10655, t10657, t10671, t10673, t10674, t10676)
}
