//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2971/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2971<F: Float>(t18900: F, t4719: F, t78328: F, t78332: F, t78335: F, t78339: F, t78342: F, t78703: F, t78706: F, t78709: F, t78712: F, t78715: F) -> (F, F) {
    let t78717 = F::cast_from(0.30762056574649219972e4_f64) * t4719 * t18900;
    let t78718 = -t78328 + t78332 + t78335 + t78339 + t78342 - t78703 - t78706 + t78709 - t78712 + t78715 - t78717;
    (t78717, t78718)
}
