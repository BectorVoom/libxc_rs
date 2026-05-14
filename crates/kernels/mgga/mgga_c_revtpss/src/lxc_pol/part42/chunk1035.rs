//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1035/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1035<F: Float>(t14207: F, t2782: F, t5659: F, t72: F, t686: F, t4101: F, t136: F, t1883: F, t2457: F, t10139: F, t13926: F, t543: F, t4100: F, t10014: F, t5741: F, t13790: F, t1398: F) -> (F, F, F, F, F, F) {
    let t14209 = 0.10975748638225852664e-1 * t2782 * t14207;
    let t14215 = t5659 * t72;
    let t14216 = t14215 * t686;
    let t14218 = 0.19514881078765566038e-1 * t4101 * t14216;
    let t14219 = t1883 * t136;
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14224 = t13926 * t543;
    let t14225 = t4100 * t14224;
    let t14227 = 0.10975748638225852664e-1 * t2782 * t14225;
    let t14229 = 0.19514881078765566038e-1 * t10014 * t5741;
    let t14230 = t13790 * t1398;
    (t14209, t14218, t14221, t14227, t14229, t14230)
}
