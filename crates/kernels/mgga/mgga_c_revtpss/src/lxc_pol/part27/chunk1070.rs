//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1070/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1070<F: Float>(t218: F, t816: F, t92993: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10680: F, t1945: F, t807: F, t10690: F, t9646: F, t10674: F, t7030: F, t9789: F) -> (F, F, F, F, F, F, F, F) {
    let t92995 = t92993 * t218 * t816;
    let t92996 = 455.0 / 1296.0 * t92995;
    let t92997 = t1946 * t10685;
    let t92998 = 0.7558530601555998074e-1 * t92997;
    let t92999 = t7033 * t10671;
    let t93000 = 0.25692334753583138159e-2 * t92999;
    let t93001 = t2689 * t25255;
    let t93004 = t807 * t1945 * t10680;
    let t93007 = t9646 * t1945 * t10690;
    let t93008 = 0.4016411544023718989e-6 * t93007;
    let t93010 = t807 * t1945 * t10674;
    let t93012 = t9789 * t7030;
    (t92996, t92998, t93000, t93001, t93004, t93008, t93010, t93012)
}
