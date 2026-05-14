//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1288/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1288<F: Float>(t120009: F, t2185: F, t23657: F, t27152: F, t27165: F, t2: F, t30105: F, t1969: F, t379: F, t5899: F, t28: F, t4668: F, t89: F, t94208: F, t23925: F, t4714: F) -> (F, F, F, F, F) {
    let t120010 = t120009 / 18.0;
    let t120013 = t23657 * t2185 * t27165 * t27152;
    let t120014 = t2 * t30105;
    let t120017 = t5899 * t1969 * t120014 * t379;
    let t120021 = t89 * t28 * t94208 * t4668;
    let t120025 = t89 * t28 * t23925 * t4714;
    (t120010, t120013, t120017, t120021, t120025)
}
