//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1425/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1425<F: Float>(t555: F, t6861: F, t6843: F, t1398: F, t9994: F, t550: F, t543: F, t3992: F, t2661: F, t4003: F, t9934: F, t3989: F, t6856: F) -> (F, F, F, F, F, F, F) {
    let t22005 = t555 * t6861;
    let t22009 = t555 * t6843;
    let t22016 = t9994 * t1398;
    let t22020 = t550 * t6843;
    let t22021 = t22020 * t543;
    let t22022 = t3992 * t22021;
    let t22023 = t2661 * t22022;
    let t22025 = t550 * t6861;
    let t22026 = t22025 * t4003;
    let t22027 = t9934 * t22026;
    let t22028 = t2661 * t22027;
    let t22030 = t3989 * t6856;
    (t22005, t22009, t22016, t22023, t22025, t22028, t22030)
}
