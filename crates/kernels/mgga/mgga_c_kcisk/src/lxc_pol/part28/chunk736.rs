//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 736/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk736<F: Float>(t776: F, t5508: F, t9176: F, t1586: F, t20: F, t8857: F, t780: F, t2629: F, t2633: F, t41: F, t8616: F, t2442: F, t2620: F, t525: F, t642: F, t773: F, t8781: F, t8787: F) -> (F, F, F, F, F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t9177 = t5508 * t9176;
    let t9178 = t1586 * t9177;
    let t9183 = t8857 * t20;
    let t9184 = t780 * t9183;
    let t9189 = t2629 * t2633;
    let t9192 = t8616 * t41;
    let t9206 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t9192 * t642 - 20.0 / 27.0 * t525 * t2620 * t2442 + 40.0 / 81.0 * t525 * t773 * t8781 - 10.0 / 27.0 * t525 * t773 * t8787);
    (t9177, t9178, t9183, t9184, t9189, t9192, t9206)
}
