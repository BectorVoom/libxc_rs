//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 763/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk763<F: Float>(t1131: F, t7553: F, t729: F, t762: F, t1168: F, t2568: F, t242: F, t2574: F, t265: F, t35353: F, t1456: F, t6852: F, t35323: F, t10157: F, t35318: F, t33291: F, t33318: F, t35312: F, t35316: F, t35321: F, t35326: F, t35330: F, t35334: F, t35338: F, t35341: F, t35346: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35634 = t7553 * t1131;
    let t35636 = t729 * t762 * t35634;
    let t35639 = t7553 * t1168;
    let t35640 = t2568 * t35639;
    let t35641 = t242 * t35640;
    let t35645 = t2574 * t265 * t35353;
    let t35649 = t2574 * t1456 * t6852;
    let t35653 = t2574 * t265 * t35323;
    let t35657 = t10157 * t265 * t35318;
    let t35669 = 3.0 / 2.0 * t35312 + t33291 + 2.0 / 3.0 * t35316 + 4.0 * t35321 - 2.0 * t35326 - t35330 / 2.0 - t33318 - t35334 / 3.0 - 3.0 * t35338 + 2.0 * t35341 + t35346 / 4.0;
    (t35634, t35636, t35639, t35640, t35641, t35645, t35649, t35653, t35657, t35669)
}
