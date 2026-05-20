//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1191/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1191<F: Float>(t2777: F, t5759: F, t2439: F, t1398: F, t1892: F, t4086: F, t543: F, t2782: F, t5659: F, t72: F, t686: F, t4101: F) -> (F, F, F) {
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    let t14207 = t4086 * t1892 * t1398 * t543;
    let t14209 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14207;
    let t14215 = t5659 * t72;
    let t14216 = t14215 * t686;
    let t14218 = F::cast_from(0.19514881078765566038e-1_f64) * t4101 * t14216;
    (t14203, t14209, t14218)
}
