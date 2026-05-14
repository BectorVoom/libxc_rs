//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 846/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk846<F: Float>(t1240: F, t2770: F, t2877: F, t848: F, t2884: F, t15143: F, t15147: F, t15150: F, t15154: F, t15159: F, t15164: F, t15168: F, t15170: F, t15172: F, t15177: F, t15180: F, t15185: F, t15190: F, t1901: F, t193: F, t446: F, t89: F) -> (F,) {
    let t15191 = t2770 * t1240;
    let t15192 = t15191 * t2877;
    let t15195 = t848 * t1240;
    let t15196 = t15195 * t2884;
    let t15199 = t89 * t193 * t15143 / 3.0 - 4.0 / 27.0 * t15147 - t446 * t15150 / 9.0 + 2.0 / 3.0 * t446 * t15154 + 2.0 / 3.0 * t446 * t15159 + t446 * t15164 / 3.0 - t15168 - t15170 + 4.0 / 3.0 * t446 * t15172 + 4.0 / 3.0 * t446 * t15177 - 22.0 / 27.0 * t15180 - 2.0 / 9.0 * t1901 * t15185 - t15190 + 2.0 / 9.0 * t1901 * t15192 + 2.0 / 9.0 * t1901 * t15196;
    (t15199,)
}
