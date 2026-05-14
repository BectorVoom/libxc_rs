//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 580/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk580<F: Float>(t1882: F, t3010: F, t2989: F, t2994: F, t2985: F, t7775: F, t8192: F, t7773: F, t89: F, t921: F, t3104: F, t375: F, t1636: F, t943: F, t383: F, t7857: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10992 = t1882 * t3010;
    let t10993 = t10992 / 27.0;
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / 27.0;
    let t11023 = t1882 * t2994;
    let t11024 = 2.0 / 27.0 * t11023;
    let t11025 = t1882 * t2985;
    let t11026 = 2.0 / 81.0 * t11025;
    let t11027 = 4.0 / 81.0 * t7775;
    let t11031 = 4.0 / 27.0 * t8192;
    let t11043 = t89 * t7773 * t921;
    let t11069 = t89 * t375 * t3104;
    let t11070 = t11069 / 9.0;
    let t11076 = t89 * t1636 * t943;
    let t11119 = t7857 * t383;
    (t10992, t10993, t11021, t11022, t11023, t11024, t11025, t11026, t11027, t11031, t11043, t11069, t11070, t11076, t11119)
}
