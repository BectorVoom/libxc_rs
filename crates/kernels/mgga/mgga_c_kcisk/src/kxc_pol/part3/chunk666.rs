//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 666/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk666<F: Float>(t10552: F, t1685: F, t4787: F, t1640: F, t4703: F, t4706: F, t1663: F, t4705: F, t1664: F, t4742: F, t1665: F, t4736: F) -> (F, F, F, F, F) {
    let t10554 = t4787 * t10552 * t1685;
    let t10557 = t1640 * t4703;
    let t10559 = F::new(6.0) * t10557 * t4706;
    let t10560 = t4705 * t1663;
    let t10561 = t10560 * t1664;
    let t10563 = F::new(6.0) * t4742 * t10561;
    let t10564 = t1665 * t4736;
    (t10554, t10559, t10560, t10563, t10564)
}
