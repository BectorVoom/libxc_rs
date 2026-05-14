//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1095/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1095<F: Float>(t776: F, t25052: F, t5491: F, t1775: F, t23220: F, t5497: F, t22919: F, t41: F, t2399: F, t260: F, t67: F, t18309: F, t23733: F, t23304: F, t7568: F, t1758: F, t1995: F, t23743: F, t2442: F, t525: F, t642: F, t7556: F, t7567: F, t7569: F, t773: F, t8781: F, t8787: F, t9192: F) -> (F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t25053 = t5491 * t25052;
    let t25054 = t1775 * t25053;
    let t25057 = t5497 * t23220;
    let t25058 = t1775 * t25057;
    let t25063 = t22919 * t41;
    let t25074 = t260 * t67 * t2399;
    let t25080 = t18309 * t23733;
    let t25086 = t7568 * t23304;
    let t25093 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t25063 * t642 - 10.0 / 27.0 * t525 * t9192 * t1758 - 20.0 / 27.0 * t525 * t7556 * t2442 + 80.0 / 81.0 * t25074 * t7569 + 40.0 / 81.0 * t525 * t1995 * t8781 - 280.0 / 243.0 * t7567 * t25080 - 10.0 / 27.0 * t525 * t1995 * t8787 + 40.0 / 81.0 * t7567 * t25086 - 10.0 / 27.0 * t525 * t773 * t23743);
    (t25054, t25058, t25093)
}
