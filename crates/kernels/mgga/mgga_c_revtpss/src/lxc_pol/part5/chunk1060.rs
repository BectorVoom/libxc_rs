//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1060/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1060<F: Float>(t14124: F, t2782: F, t555: F, t5658: F, t4086: F, t543: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F) -> (F, F, F, F) {
    let t14126 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14124;
    let t14127 = t555 * t5658;
    let t14129 = t4086 * t14127 * t543;
    let t14131 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14129;
    let t14140 = t4114 * t1882;
    let t14141 = t2482 * t14140;
    let t14143 = t4003 * t72 * t122;
    (t14126, t14131, t14141, t14143)
}
