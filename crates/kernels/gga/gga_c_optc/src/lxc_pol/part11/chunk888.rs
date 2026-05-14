//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 888/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk888<F: Float>(t17340: F, t3284: F, t914: F, t17449: F, t8749: F, t3061: F, t1102: F, t11671: F, t11677: F, t14881: F, t14883: F, t14885: F, t14887: F, t14889: F, t14895: F, t17381: F, t17384: F, t17389: F, t17392: F, t17394: F, t8727: F) -> (F, F, F, F, F) {
    let t17464 = t3284 * t17340;
    let t17465 = t914 * t17464;
    let t17468 = t8749 * t17449;
    let t17469 = t17468 * t3061;
    let t17471 = 0.1038945353962551798e3 * t1102 * t17469;
    let t17485 = -0.33114e0 * t14881 + 0.16557e0 * t14883 + 0.20128333333333333333e0 * t14885 - 0.60385000000000000001e0 * t14887 + 0.30192500000000000001e0 * t14889 + 0.5519e-1 * t14895 + 0.258925e1 * t17381 + 0.19419375e1 * t17384 - 0.40256666666666666668e0 * t11671 - 0.27595e0 * t11677 - 0.82785e-1 * t17389 + 0.49671e0 * t17392 - 0.412621875e-1 * t17394 - t8727;
    (t17464, t17465, t17469, t17471, t17485)
}
