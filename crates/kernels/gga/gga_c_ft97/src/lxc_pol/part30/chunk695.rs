//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 695/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk695<F: Float>(t242: F, t33490: F, t1882: F, t7555: F, t2574: F, t265: F, t33346: F, t7484: F, t766: F, t729: F, t762: F, t33602: F, t713: F, t7546: F, t2568: F, t33599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33654 = t242 * t33490;
    let t33658 = t1882 * t7555 / 9.0;
    let t33660 = t2574 * t265 * t33346;
    let t33663 = t7484 * t766;
    let t33665 = t729 * t762 * t33663;
    let t33668 = t242 * t33602;
    let t33671 = t7546 * t713;
    let t33673 = t729 * t2568 * t33671;
    let t33676 = t242 * t33599;
    (t33654, t33658, t33660, t33663, t33665, t33668, t33671, t33673, t33676)
}
