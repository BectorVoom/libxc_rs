//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2001/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2001<F: Float>(t14114: F, t4104: F, t10073: F, t5737: F, t1419: F, t1882: F, t4086: F, t543: F, t2782: F, t555: F, t5658: F, t4114: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14116 = F::cast_from(0.19514881078765566038e-1_f64) * t14114 * t4104;
    let t14120 = t10073 * t5737;
    let t14122 = t1419 * t1882;
    let t14124 = t4086 * t14122 * t543;
    let t14126 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14124;
    let t14127 = t555 * t5658;
    let t14129 = t4086 * t14127 * t543;
    let t14131 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14129;
    let t14140 = t4114 * t1882;
    (t14116, t14120, t14122, t14124, t14126, t14127, t14129, t14131, t14140)
}
