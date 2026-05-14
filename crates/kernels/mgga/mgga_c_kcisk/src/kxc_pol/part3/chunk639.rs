//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 639/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk639<F: Float>(t4786: F, t596: F, t10552: F, t4790: F, t1675: F, t4789: F, t599: F, t1644: F, t4696: F, t1665: F, t4699: F, t4737: F, t1640: F, t4741: F, t4745: F, t4740: F, t583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10690 = 1.0 / t4786 / t596;
    let t10692 = t10690 * t10552 * t4790;
    let t10696 = 1.0 / t4786 / t1675;
    let t10699 = 1.0 / t4789 / t599;
    let t10700 = t10696 * t10552 * t10699;
    let t10705 = t4696 * t1644;
    let t10707 = 3.0 * t10705 * t1665;
    let t10709 = 3.0 * t4699 * t4737;
    let t10710 = t1640 * t4741;
    let t10712 = 0.48245472966453314466e2 * t10710 * t4745;
    let t10714 = 1.0 / t4740 / t583;
    (t10690, t10692, t10696, t10699, t10700, t10707, t10709, t10712, t10714)
}
