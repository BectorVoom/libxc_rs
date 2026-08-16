//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1194/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1194<F: Float>(t30735: F, t7637: F, t2142: F, t6573: F, t1769: F, t8190: F, t1774: F, t6563: F, t1828: F, t8201: F, t7652: F, t1794: F, t8208: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30736 = t7637 * t30735;
    let t30739 = t2142 * t6573;
    let t30740 = t7637 * t30739;
    let t30743 = t8190 * t1769;
    let t30744 = t7637 * t30743;
    let t30747 = t8190 * t1774;
    let t30748 = t7637 * t30747;
    let t30751 = t2142 * t6563;
    let t30752 = t7637 * t30751;
    let t30757 = t8201 * t1828;
    let t30758 = t7652 * t30757;
    let t30763 = t8208 * t1794;
    (t30736, t30739, t30740, t30744, t30747, t30748, t30751, t30752, t30758, t30763)
}
