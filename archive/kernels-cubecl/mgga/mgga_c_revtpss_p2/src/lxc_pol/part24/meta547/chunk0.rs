//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1620/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1620<F: Float>(t14330: F, t18305: F, t5819: F, t190: F, t2611: F, t87107: F, t23121: F, t50089: F, t50084: F, t50092: F, t50094: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t40137: F) -> (F, F, F, F, F, F, F) {
    let t87655 = F::cast_from(144.0_f64) * t14330 * t18305 * t5819;
    let t87658 = F::cast_from(36.0_f64) * t2611 * t190 * t87107;
    let t87660 = F::cast_from(96.0_f64) * t50089 * t23121;
    let t87661 = F::cast_from(16.0_f64) * t50084;
    let t87662 = F::cast_from(0.65061487801810439052e-1_f64) * t50092;
    let t87663 = F::cast_from(0.19263893255070628431e1_f64) * t50094;
    let t87664 = t40088 + t40099 + t40103 + t87655 - t40115 + t87658 + t87660 - t40131 - t40137 + t87661 + t87662 + t87663;
    (t87655, t87658, t87660, t87661, t87662, t87663, t87664)
}
