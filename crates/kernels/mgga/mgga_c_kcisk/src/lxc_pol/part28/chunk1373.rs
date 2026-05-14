//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1373/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1373<F: Float>(t116223: F, t1869: F, t34085: F, t1772: F, t4823: F, t8792: F, t35243: F, t9660: F, t35146: F, t1894: F, t33017: F, t9035: F, t1757: F, t8787: F, t9679: F, t34073: F, t34097: F) -> (F, F, F, F, F, F, F) {
    let t121662 = t1869 * t116223 * t34085;
    let t121667 = t4823 * t8792 * t1772;
    let t121671 = t35243 * t9660;
    let t121673 = t35146 * t9660;
    let t121679 = t1869 * t33017 * t9035 * t1894;
    let t121683 = t1869 * t9679 * t8787 * t1757;
    let t121685 = t34073 * t34097;
    (t121662, t121667, t121671, t121673, t121679, t121683, t121685)
}
