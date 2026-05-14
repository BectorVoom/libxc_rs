//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1168/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1168<F: Float>(t33568: F, t10847: F, t22693: F, t7572: F, t24554: F, t959: F, t1: F, t33137: F, t2021: F, t20671: F, t22538: F, t24549: F, t10984: F, t11033: F, t1445: F, t1457: F, t2004: F, t2005: F, t2178: F, t28715: F, t32180: F, t32230: F, t33544: F, t33546: F, t33560: F, t33564: F, t33567: F, t5703: F, t833: F) -> (F,) {
    let t33569 = 0.29792074959875355558e-1 * t33568;
    let t33572 = 0.18404604457881959845e2 * t7572 * t22693 * t10847;
    let t33573 = t24554 * t959;
    let t33574 = 0.14896037479937677779e-1 * t33573;
    let t33575 = t33137 * t1;
    let t33576 = t2021 * t33575;
    let t33580 = t22538 * t20671 * t24549;
    let t33581 = 0.85206502119823888168e-1 * t33580;
    let t33582 = -t33544 - t33546 + t28715 + 0.43710935587469654631e2 * t833 * t1445 * t32230 + 0.46011511144704899612e1 * t2178 * t11033 + 0.71500979903700853338e0 * t2004 * t1457 * t32180 + 0.71500979903700853338e0 * t5703 * t10984 - t33560 + t33564 + t33567 + t33569 + t33572 + t33574 + 0.21450293971110256002e1 * t33576 * t2005 + t33581;
    (t33582,)
}
