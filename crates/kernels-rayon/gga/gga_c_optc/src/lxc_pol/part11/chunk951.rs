//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 951/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk951(t17340: f64, t3284: f64, t914: f64, t17449: f64, t8749: f64, t3061: f64, t1102: f64, t11671: f64, t11677: f64, t14881: f64, t14883: f64, t14885: f64, t14887: f64, t14889: f64, t14895: f64, t17381: f64, t17384: f64, t17389: f64, t17392: f64, t17394: f64, t8727: f64) -> (f64, f64, f64, f64, f64) {
    let t17464 = t3284 * t17340;
    let t17465 = t914 * t17464;
    let t17468 = t8749 * t17449;
    let t17469 = t17468 * t3061;
    let t17471 = 0.1038945353962551798e3_f64 * t1102 * t17469;
    let t17485 = -0.33114e0_f64 * t14881 + 0.16557e0_f64 * t14883 + 0.20128333333333333333e0_f64 * t14885 - 0.60385000000000000001e0_f64 * t14887 + 0.30192500000000000001e0_f64 * t14889 + 0.5519e-1_f64 * t14895 + 0.258925e1_f64 * t17381 + 0.19419375e1_f64 * t17384 - 0.40256666666666666668e0_f64 * t11671 - 0.27595e0_f64 * t11677 - 0.82785e-1_f64 * t17389 + 0.49671e0_f64 * t17392 - 0.412621875e-1_f64 * t17394 - t8727;
    (t17464, t17465, t17469, t17471, t17485)
}
