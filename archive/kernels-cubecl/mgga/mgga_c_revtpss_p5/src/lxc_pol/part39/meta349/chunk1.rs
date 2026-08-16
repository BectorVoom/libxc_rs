//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1187/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1187<F: Float>(t14127: F, t4086: F, t543: F, t2782: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F) -> (F, F, F, F) {
    let t14129 = t4086 * t14127 * t543;
    let t14131 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14129;
    let t14140 = t4114 * t1882;
    let t14141 = t2482 * t14140;
    let t14143 = t4003 * t72 * t122;
    let t14144 = t676 * t1398;
    (t14131, t14141, t14143, t14144)
}
