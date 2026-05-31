//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1192/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1192<F: Float>(t95574: F, t95622: F, t95729: F, t95776: F, t95821: F, t95863: F, t95904: F, t95950: F, t892: F, t2070: F, t41154: F, t1940: F, t2403: F, t25198: F, t25208: F, t25449: F, t26425: F, t26581: F, t26585: F, t28291: F, t30: F, t4541: F, t7010: F, t7092: F, t7428: F, t7432: F, t92743: F, t92753: F, t92759: F, t92765: F, t92768: F, t92772: F, t92779: F, t92791: F, t92810: F, t95511: F, t95527: F) -> (F, F, F, F) {
    let t95953 = t95574 + t95622 + t95729 + t95776 + t95821 + t95863 + t95904 + t95950;
    let t95954 = t95953 * t892;
    let t95964 = t2070 * t41154;
    let t95972 = -F::cast_from(9.0_f64) * t95511 * t25208 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t26425 * t92765 - F::cast_from(9.0_f64) * t26425 * t92791 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7432 * t92779 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7432 * t92768 - F::cast_from(3.0_f64) * t1940 * t26585 * t25449 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t95527 * t7092 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t26425 * t92759 + F::cast_from(9.0_f64) * t4541 * t7428 * t25198 + t1940 * t95954 * t30 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2403 * t26581 * t7010 - t1940 * t7432 * t92810 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t1940 * t95964 * t92743 - F::cast_from(9.0_f64) * t28291 * t92753 + F::cast_from(9.0_f64) * t28291 * t92772;
    (t95953, t95954, t95964, t95972)
}
