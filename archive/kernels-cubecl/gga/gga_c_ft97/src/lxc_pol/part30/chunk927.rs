//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 927/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk927<F: Float>(t2842: F, t7021: F, t14763: F, t7005: F, t22511: F, t33939: F, t4113: F, t7003: F, t19100: F, t4061: F, t19116: F, t280: F, t5009: F) -> (F, F, F, F, F, F, F) {
    let t126613 = t2842 * t7021;
    let t127111 = t14763 * t7005;
    let t127359 = t33939 * t22511;
    let t127360 = t4113 * t127359;
    let t127456 = t7003 * t127359;
    let t127560 = t4061 * t19100;
    let t127614 = t19116 * t19100;
    let t127649 = t280 * t5009;
    (t126613, t127111, t127360, t127456, t127560, t127614, t127649)
}
