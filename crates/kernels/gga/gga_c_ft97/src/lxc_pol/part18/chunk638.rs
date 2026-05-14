//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 638/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk638<F: Float>(t11064: F, t7824: F, t446: F, t3104: F, t375: F, t89: F, t358: F, t463: F, t2999: F, t1636: F, t943: F, t3057: F, t401: F, t1595: F, t930: F, t7914: F) -> (F, F, F, F, F, F, F, F) {
    let t11065 = t7824 * t11064;
    let t11066 = t446 * t11065;
    let t11069 = t89 * t375 * t3104;
    let t11070 = t11069 / 9.0;
    let t11071 = t463 * t358;
    let t11073 = t89 * t2999 * t11071;
    let t11076 = t89 * t1636 * t943;
    let t11080 = t3057 * t401;
    let t11084 = t930 * t1595;
    let t11085 = t7914 * t11084;
    (t11066, t11069, t11070, t11073, t11076, t11080, t11084, t11085)
}
