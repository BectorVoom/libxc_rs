//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 786/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk786<F: Float>(t3508: F, t6272: F, t3507: F, t1662: F, t1851: F, t3515: F, t3520: F, t1252: F, t1253: F, t6276: F, t3531: F, t286: F, t3537: F, t4612: F, t6328: F, t6332: F, t6336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6758 = t3508 * t6272;
    let t6759 = t3507 * t6758;
    let t6762 = t1662 * t1851;
    let t6763 = t3515 * t6762;
    let t6766 = t3520 * t6272;
    let t6767 = t1252 * t6766;
    let t6770 = t1253 * t6276;
    let t6771 = t1252 * t6770;
    let t6774 = t1851 * t1851;
    let t6775 = t3531 * t6774;
    let t6776 = t286 * t6775;
    let t6783 = t3537 + 0.11415555555555555555e-1 * t4612 - 0.11415555555555555555e-1 * t6328 + 0.34246666666666666666e-1 * t6332 - 0.17123333333333333333e-1 * t6336;
    (t6758, t6759, t6762, t6763, t6766, t6767, t6770, t6771, t6774, t6775, t6776, t6783)
}
