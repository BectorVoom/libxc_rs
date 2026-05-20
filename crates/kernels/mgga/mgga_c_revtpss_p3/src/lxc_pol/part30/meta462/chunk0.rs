//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1758/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1758<F: Float>(t2035: F, t25188: F, t531: F, t7311: F, t7238: F, t2014: F, t7312: F, t7315: F, t2394: F, t30: F, t1962: F, t198: F, t206: F) -> (F, F, F, F, F, F, F, F) {
    let t25189 = t25188 * t2035;
    let t25190 = t531 * t7311;
    let t25191 = t25190 * t7238;
    let t25193 = F::new(6.0) * t2014 * t25191;
    let t25194 = t7312 * t7315;
    let t25196 = F::new(2.0) * t2014 * t25194;
    let t25198 = t30 * t2394;
    let t25206 = t198 * t206 * t1962;
    (t25189, t25190, t25191, t25193, t25194, t25196, t25198, t25206)
}
