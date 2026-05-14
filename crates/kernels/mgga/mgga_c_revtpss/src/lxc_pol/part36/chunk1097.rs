//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1097/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1097<F: Float>(t92995: F, t10685: F, t1946: F, t10671: F, t7033: F, t10690: F, t1945: F, t9646: F, t7030: F, t9789: F, t2453: F, t2783: F, t64: F, t9784: F, t2482: F, t25260: F, t27: F) -> (F, F, F, F, F, F, F, F) {
    let t92996 = 455.0 / 1296.0 * t92995;
    let t92997 = t1946 * t10685;
    let t92998 = 0.7558530601555998074e-1 * t92997;
    let t92999 = t7033 * t10671;
    let t93000 = 0.25692334753583138159e-2 * t92999;
    let t93007 = t9646 * t1945 * t10690;
    let t93008 = 0.4016411544023718989e-6 * t93007;
    let t93012 = t9789 * t7030;
    let t93013 = 0.22589491248727328397e-6 * t93012;
    let t93015 = t2453 * t2783 * t64;
    let t93020 = t9784 * t7030;
    let t93021 = 0.14450132032386466905e-2 * t93020;
    let t93025 = t2482 * t25260 * t27;
    (t92996, t92998, t93000, t93008, t93013, t93015, t93021, t93025)
}
