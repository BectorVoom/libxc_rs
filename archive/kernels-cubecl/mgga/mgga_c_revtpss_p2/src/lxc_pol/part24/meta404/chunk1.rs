//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1342/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1342<F: Float>(t225: F, t40603: F, t785: F, t2737: F, t853: F, t9794: F, t10292: F, t66: F, t240: F, t10688: F, t243: F, t268: F) -> (F, F, F, F, F, F) {
    let t40609 = t40603 * t785 * t225;
    let t40611 = F::cast_from(0.63807336860547134325e-3_f64) * t40609 * t2737;
    let t40627 = t9794 * t853;
    let t40633 = F::cast_from(1.0_f64) / t66 / t10292;
    let t40634 = t40633 * t240;
    let t40638 = F::cast_from(0.53552153920316253184e-5_f64) * t10688 * t40634 * t243 * t268;
    (t40609, t40611, t40627, t40633, t40634, t40638)
}
