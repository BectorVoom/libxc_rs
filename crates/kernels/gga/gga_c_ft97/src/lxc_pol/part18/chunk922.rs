//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 922/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk922<F: Float>(t24028: F, t24045: F, t143: F, t160: F, t1380: F, t1637: F, t89: F, t1901: F, t23953: F, t23957: F, t23961: F, t23965: F, t23970: F, t23974: F, t23978: F, t23982: F, t23986: F, t23990: F, t23994: F, t23999: F, t24003: F, t24004: F, t24007: F, t24010: F, t28: F, t446: F) -> (F, F, F, F) {
    let t24046 = t24028 + t24045;
    let t24048 = t143 * t24046 * t160;
    let t24054 = 4.0 / 27.0 * t89 * t1637 * t1380;
    let t24055 = 2.0 / 3.0 * t446 * t23953 - t446 * t23957 / 3.0 - 2.0 / 3.0 * t446 * t23961 + 4.0 / 3.0 * t446 * t23965 + t446 * t23970 / 3.0 + 2.0 / 3.0 * t446 * t23974 - t446 * t23978 / 3.0 - 2.0 / 3.0 * t446 * t23982 - t446 * t23986 / 3.0 - 2.0 / 3.0 * t446 * t23990 - t446 * t23994 / 3.0 + 2.0 / 3.0 * t446 * t23999 - t24003 + 2.0 / 9.0 * t24004 - 2.0 / 9.0 * t24007 - 2.0 / 9.0 * t1901 * t24010 + t89 * t28 * t24048 / 3.0 + t24054;
    (t24046, t24048, t24054, t24055)
}
