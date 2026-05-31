//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2848/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2848<F: Float>(t61090: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t49877: F, t76890: F, t76893: F, t76932: F, t76935: F, t76936: F, t76937: F, t76938: F, t76939: F, t76940: F, t76941: F) -> (F, F) {
    let t76942 = F::cast_from(12.0_f64) * t61090;
    let t76943 = t76890 + t76893 + t76932 - t39419 - t39422 + t76935 + t76936 - t76937 - t76938 - t76939 - t39429 - t39432 + t76940 + t76941 + t39442 + t49877 + t76942;
    (t76942, t76943)
}
