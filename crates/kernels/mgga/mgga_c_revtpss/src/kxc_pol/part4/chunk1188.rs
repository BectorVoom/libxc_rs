//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1188/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1188<F: Float>(t10815: F, t1561: F, t2741: F, t4426: F, t10845: F, t4430: F, t1558: F, t853: F, t2749: F, t2662: F, t2661: F, t4352: F, t837: F) -> (F, F, F, F, F) {
    let t14712 = t10815 * t1561;
    let t14715 = F::cast_from(0.20007875121765877254e-2_f64) * t2741 * t4426;
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14719 = t14718 * t2749;
    let t14720 = t2662 * t14719;
    let t14722 = F::cast_from(0.57165357490759649296e-4_f64) * t2661 * t14720;
    let t14723 = t4352 * t837;
    (t14712, t14715, t14716, t14722, t14723)
}
