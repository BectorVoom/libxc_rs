//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 384/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk384<F: Float>(t3626: F, t531: F, t3630: F, t3601: F, t808: F, t568: F, t836: F, t3614: F, t2090: F, t2087: F, t2098: F, t2103: F, t317: F, t3309: F, t3468: F, t3475: F, t3490: F, t3501: F, t3642: F, t3646: F, t3651: F, t797: F, t813: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3655 = t531 * t3626;
    let t3658 = t531 * t3630;
    let t3661 = t808 * t3601;
    let t3662 = t568 * t3661;
    let t3666 = t836 * t3601;
    let t3667 = t568 * t3666;
    let t3670 = t808 * t3614;
    let t3671 = t568 * t3670;
    let t3676 = t2090 * t3601;
    let t3677 = t568 * t3676;
    let t3680 = t836 * t3614;
    let t3681 = t568 * t3680;
    let t3684 = 0.35750489951850426669e0 * t3642 * t317 + 0.35750489951850426669e0 * t3646 * t317 + 0.59584149919750711116e-1 * t3468 - 0.10725146985555128001e1 * t3651 * t2098 - 0.59584149919750711116e-1 * t3475 + 0.71500979903700853338e0 * t2103 * t3655 - 0.35750489951850426669e0 * t797 * t3658 - 0.46011511144704899612e1 * t813 * t3662 - 0.76685851907841499353e0 * t3490 + 0.11502877786176224903e2 * t833 * t3667 - 0.23005755572352449806e1 * t813 * t3671 - 0.31952438294933958063e-1 * t3309 + 0.76685851907841499353e0 * t3501 - 0.69017266717057349418e1 * t2087 * t3677 + 0.23005755572352449806e1 * t833 * t3681;
    (t3655, t3658, t3661, t3662, t3666, t3667, t3670, t3671, t3676, t3677, t3680, t3681, t3684)
}
