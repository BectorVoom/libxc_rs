//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 283/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk283<F: Float>(t1113: F, t237: F, t1100: F, t1103: F, t14: F, t231: F, t228: F, t704: F, t992: F, t420: F, t701: F, t699: F) -> (F, F, F, F, F) {
    let t1114 = t237 * t1113;
    let t1115 = t1100 * t1114;
    let t1119 = t1103 * t14;
    let t1120 = t1119 * t231;
    let t1121 = t228 * t1120;
    let t1123 = t704 * t992;
    let t1124 = t420 * t1123;
    let t1125 = t701 * t1124;
    let t1127 = -F::cast_from(0.51074886703703703704e-1_f64) * t1121 + t699 + F::cast_from(0.6384360837962962963e-2_f64) * t1125;
    (t1115, t1121, t1123, t1125, t1127)
}
