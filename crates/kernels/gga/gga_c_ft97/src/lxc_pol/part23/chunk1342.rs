//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1342/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1342<F: Float>(t126795: F, t24976: F, t6317: F, t19460: F, t25037: F, t43350: F, t446: F, t10248: F, t125858: F, t125862: F, t125866: F, t3281: F, t113106: F, t126781: F, t126784: F, t126787: F, t126790: F, t126793: F, t99736: F) -> (F, F, F, F, F, F, F) {
    let t126797 = t6317 * t24976 * t126795;
    let t126799 = t25037 * t19460;
    let t126801 = t446 * t43350 * t126799;
    let t126804 = t446 * t10248 * t125858;
    let t126807 = t446 * t10248 * t125862;
    let t126810 = t3281 * t10248 * t125866;
    let t126812 = 2.0 / 9.0 * t126781 + t126784 - 12.0 * t126787 + t126790 - 4.0 / 3.0 * t126793 + t113106 + t99736 - 2.0 / 3.0 * t126797 - 4.0 / 9.0 * t126801 - 4.0 / 3.0 * t126804 - 4.0 / 3.0 * t126807 + 8.0 / 3.0 * t126810;
    (t126797, t126799, t126801, t126804, t126807, t126810, t126812)
}
