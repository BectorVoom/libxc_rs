//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 692/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk692<F: Float>(t7546: F, t766: F, t10052: F, t1449: F, t6187: F, t2568: F, t6154: F, t6166: F, t729: F, t24412: F, t6175: F, t242: F, t684: F, t724: F, t7560: F, t10157: F, t265: F, t33302: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33598 = t7546 * t766;
    let t33599 = t10052 * t33598;
    let t33601 = t1449 * t6187;
    let t33602 = t2568 * t33601;
    let t33605 = t729 * t6154 * t6166;
    let t33608 = t24412 * t6175;
    let t33609 = t242 * t33608;
    let t33613 = t724 * t7560 * t684;
    let t33617 = t10157 * t265 * t33302;
    (t33598, t33599, t33601, t33602, t33605, t33608, t33609, t33613, t33617)
}
