//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 888/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk888<F: Float>(t10691: F, t7064: F, t5539: F, t8769: F, t9647: F, t123: F, t8773: F, t2563: F, t2558: F, t8788: F, t1843: F, t8756: F, t3440: F, t7137: F, t3420: F, t8469: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10692 = t7064 * t10691;
    let t10693 = 0.32043859292259267849e-3 * t10692;
    let t10694 = t5539 * t8769;
    let t10695 = t9647 * t10694;
    let t10696 = 0.64087718584518535698e-3 * t10695;
    let t10697 = t8773 * t123;
    let t10698 = t10697 * t2563;
    let t10699 = t9647 * t10698;
    let t10700 = 0.96131577876777803547e-3 * t10699;
    let t10701 = t8788 * t2558;
    let t10702 = t9647 * t10701;
    let t10703 = 0.32043859292259267849e-3 * t10702;
    let t10704 = t1843 * t8756;
    let t10705 = t7064 * t10704;
    let t10706 = 0.32043859292259267849e-3 * t10705;
    let t10708 = 0.30762104920568897135e-1 * t7137 * t3440;
    let t10710 = 0.10254034973522965712e-1 * t7137 * t3420;
    let t10713 = t8469 * t935;
    (t10693, t10694, t10696, t10697, t10698, t10700, t10701, t10703, t10704, t10706, t10708, t10710, t10713)
}
