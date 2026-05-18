//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 495/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk495<F: Float>(t1774: F, t9: F, t1782: F, t1849: F, t662: F, t1781: F, t661: F, t657: F, t1336: F, t140: F, t4594: F, t1870: F, t715: F) -> (F, F, F, F, F, F, F) {
    let t5014 = t9 * t1774;
    let t5015 = t5014 * t1782;
    let t5020 = t662 * t1849;
    let t5030 = F::new(1.0) / t1781 / t661;
    let t5031 = t657 * t5030;
    let t5054 = t140 * t1336 * t4594;
    let t5060 = F::new(1.0) / t1870 / t715;
    (t5014, t5015, t5020, t5030, t5031, t5054, t5060)
}
