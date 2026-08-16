//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 410/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk410(t2087: f64, t2098: f64, t2103: f64, t317: f64, t3309: f64, t3468: f64, t3475: f64, t3490: f64, t3501: f64, t3642: f64, t3646: f64, t3651: f64, t3655: f64, t3658: f64, t3662: f64, t3667: f64, t3671: f64, t3677: f64, t3681: f64, t797: f64, t813: f64, t833: f64) -> f64 {
    let t3684 = 0.35750489951850426669e0_f64 * t3642 * t317 + 0.35750489951850426669e0_f64 * t3646 * t317 + 0.59584149919750711116e-1_f64 * t3468 - 0.10725146985555128001e1_f64 * t3651 * t2098 - 0.59584149919750711116e-1_f64 * t3475 + 0.71500979903700853338e0_f64 * t2103 * t3655 - 0.35750489951850426669e0_f64 * t797 * t3658 - 0.46011511144704899612e1_f64 * t813 * t3662 - 0.76685851907841499353e0_f64 * t3490 + 0.11502877786176224903e2_f64 * t833 * t3667 - 0.23005755572352449806e1_f64 * t813 * t3671 - 0.31952438294933958063e-1_f64 * t3309 + 0.76685851907841499353e0_f64 * t3501 - 0.69017266717057349418e1_f64 * t2087 * t3677 + 0.23005755572352449806e1_f64 * t833 * t3681;
    t3684
}
