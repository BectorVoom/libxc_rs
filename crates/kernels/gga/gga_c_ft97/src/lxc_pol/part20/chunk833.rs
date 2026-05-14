//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 833/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk833<F: Float>(t1882: F, t6300: F, t6349: F, t681: F, t89: F, t6304: F, t25135: F, t319: F, t840: F, t6260: F, t882: F, t24967: F, t24971: F, t24974: F, t24978: F, t24984: F, t24987: F, t24992: F, t24995: F, t24998: F, t25003: F, t25007: F, t25010: F, t25015: F, t25020: F, t25024: F, t25031: F) -> (F, F, F, F, F, F) {
    let t25312 = t1882 * t6300;
    let t25315 = t89 * t681 * t6349;
    let t25317 = t1882 * t6304;
    let t25320 = t840 * t319 * t25135;
    let t25324 = t840 * t882 * t6260;
    let t25342 = 4.0 / 3.0 * t24967 + 2.0 / 3.0 * t24971 - t24974 / 18.0 - 2.0 / 9.0 * t24978 - t24984 / 18.0 - 4.0 / 9.0 * t24987 - 2.0 * t24992 + 2.0 / 9.0 * t24995 - 4.0 / 9.0 * t24998 + t25003 / 3.0 + 2.0 / 3.0 * t25007 - 2.0 / 9.0 * t25010 + t25015 / 12.0 + t25020 / 6.0 - t25024 - t25031 / 8.0;
    (t25312, t25315, t25317, t25320, t25324, t25342)
}
