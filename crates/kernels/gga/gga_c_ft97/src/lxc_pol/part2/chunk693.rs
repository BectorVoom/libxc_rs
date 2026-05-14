//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 693/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk693<F: Float>(t11167: F, t11177: F, t11183: F, t11192: F, t11202: F, t12233: F, t7946: F, t7948: F, t7950: F, t7952: F, t8698: F, t637: F, t639: F, t2253: F, t3655: F, t12143: F, t12144: F, t12148: F, t12152: F, t12155: F, t12158: F, t12162: F, t12164: F, t12165: F, t12171: F, t12174: F, t12177: F, t12181: F, t12186: F, t12190: F, t12193: F, t12198: F, t12201: F, t12204: F, t2265: F, t3628: F, t631: F) -> (F,) {
    let t12234 = -0.9628722222222222222e-1 * t7950 + 0.10591594444444444444e1 * t11177 - 0.28886166666666666666e0 * t11202 - t8698 + 0.3209574074074074074e-1 * t7948 - 0.12838296296296296296e0 * t7946 + 0.4814361111111111111e-1 * t7952 + 0.57772333333333333332e0 * t11183 - 0.86658499999999999998e0 * t11192 - 0.6419148148148148148e-1 * t11167 + t12233;
    let t12236 = t637 * t639 * t12234;
    let t12240 = 2.0 / 3.0 * t2253 * t3655;
    let t12241 = 2.0 / 9.0 * t12143 * t12144 - t2265 * t12148 / 3.0 - 4.0 / 3.0 * t12143 * t12152 - t2265 * t12155 / 3.0 - 4.0 / 3.0 * t12143 * t12158 + t12162 + t12164 + 5.0 / 27.0 * t12165 - 13.0 / 9.0 * t12171 + t12174 - 2.0 / 3.0 * t2265 * t12177 - t2265 * t12181 / 3.0 - t2265 * t12186 / 9.0 - t12190 - 3.0 * t631 * t12193 + 6.0 * t631 * t12198 + t3628 * t12201 / 3.0 + 5.0 / 9.0 * t12204 + t631 * t12236 / 2.0 - t12240;
    (t12241,)
}
