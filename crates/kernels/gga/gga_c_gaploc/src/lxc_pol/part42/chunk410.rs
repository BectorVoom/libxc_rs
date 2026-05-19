//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 410/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk410<F: Float>(t2087: F, t2098: F, t2103: F, t317: F, t3309: F, t3468: F, t3475: F, t3490: F, t3501: F, t3642: F, t3646: F, t3651: F, t3655: F, t3658: F, t3662: F, t3667: F, t3671: F, t3677: F, t3681: F, t797: F, t813: F, t833: F) -> F {
    let t3684 = F::cast_from(0.35750489951850426669e0_f64) * t3642 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t3646 * t317 + F::cast_from(0.59584149919750711116e-1_f64) * t3468 - F::cast_from(0.10725146985555128001e1_f64) * t3651 * t2098 - F::cast_from(0.59584149919750711116e-1_f64) * t3475 + F::cast_from(0.71500979903700853338e0_f64) * t2103 * t3655 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t3658 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t3662 - F::cast_from(0.76685851907841499353e0_f64) * t3490 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t3667 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t3671 - F::cast_from(0.31952438294933958063e-1_f64) * t3309 + F::cast_from(0.76685851907841499353e0_f64) * t3501 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t3677 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t3681;
    t3684
}
