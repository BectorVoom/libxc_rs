//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 951/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk951<F: Float>(t17340: F, t3284: F, t914: F, t17449: F, t8749: F, t3061: F, t1102: F, t11671: F, t11677: F, t14881: F, t14883: F, t14885: F, t14887: F, t14889: F, t14895: F, t17381: F, t17384: F, t17389: F, t17392: F, t17394: F, t8727: F) -> (F, F, F, F, F) {
    let t17464 = t3284 * t17340;
    let t17465 = t914 * t17464;
    let t17468 = t8749 * t17449;
    let t17469 = t17468 * t3061;
    let t17471 = F::cast_from(0.1038945353962551798e3_f64) * t1102 * t17469;
    let t17485 = -F::cast_from(0.33114e0_f64) * t14881 + F::cast_from(0.16557e0_f64) * t14883 + F::cast_from(0.20128333333333333333e0_f64) * t14885 - F::cast_from(0.60385000000000000001e0_f64) * t14887 + F::cast_from(0.30192500000000000001e0_f64) * t14889 + F::cast_from(0.5519e-1_f64) * t14895 + F::cast_from(0.258925e1_f64) * t17381 + F::cast_from(0.19419375e1_f64) * t17384 - F::cast_from(0.40256666666666666668e0_f64) * t11671 - F::cast_from(0.27595e0_f64) * t11677 - F::cast_from(0.82785e-1_f64) * t17389 + F::cast_from(0.49671e0_f64) * t17392 - F::cast_from(0.412621875e-1_f64) * t17394 - t8727;
    (t17464, t17465, t17469, t17471, t17485)
}
