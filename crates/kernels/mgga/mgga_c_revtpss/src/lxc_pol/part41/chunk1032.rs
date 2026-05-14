//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1032/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1032<F: Float>(t2435: F, t5760: F, t545: F, t5710: F, t869: F, t689: F, t225: F, t9990: F, t213: F, t2777: F, t5759: F, t2439: F, t1398: F, t1892: F, t4086: F, t543: F) -> (F, F, F, F, F) {
    let t14166 = t2435 * t5760;
    let t14188 = t545 * t5710;
    let t14189 = t869 * t14188;
    let t14191 = 0.10975748638225852664e-1 * t689 * t14189;
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    let t14207 = t4086 * t1892 * t1398 * t543;
    (t14166, t14191, t14193, t14203, t14207)
}
