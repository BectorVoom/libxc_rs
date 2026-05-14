//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1195/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1195<F: Float>(t18942: F, t2816: F, t6887: F, t406: F, t7030: F, t19004: F, t19014: F, t1411: F, t2747: F, t1385: F, t7124: F, t4990: F, t963: F, t5037: F, t2810: F, t19336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23719 = 96.0 * t18942;
    let t23731 = t6887 * t2816;
    let t23732 = 0.7089e1 * t23731;
    let t23740 = t406 * t7030;
    let t23741 = 12.0 * t23740;
    let t23752 = 36.0 * t19004;
    let t23753 = 8.0 * t19014;
    let t23758 = t2747 * t1411;
    let t23759 = 0.17544670867903938621e1 * t23758;
    let t23760 = t2747 * t1385;
    let t23761 = 0.51947577317044391276e2 * t23760;
    let t23763 = 24.0 * t406 * t7124;
    let t23764 = t963 * t4990;
    let t23768 = t963 * t5037;
    let t23774 = t6887 * t2810;
    let t23775 = 0.7089e1 * t23774;
    let t23781 = 192.0 * t19336;
    (t23719, t23732, t23741, t23752, t23753, t23759, t23761, t23763, t23764, t23768, t23775, t23781)
}
