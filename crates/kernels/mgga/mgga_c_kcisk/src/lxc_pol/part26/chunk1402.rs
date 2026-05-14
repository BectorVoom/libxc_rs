//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1402/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1402<F: Float>(t119875: F, t119877: F, t119879: F, t119881: F, t119883: F, t119885: F, t119886: F, t119901: F, t119998: F, t120006: F, t120829: F, t120836: F, t120841: F, t120844: F, t120849: F, t120851: F, t120854: F, t120855: F, t120857: F, t120877: F, t120883: F, t120886: F, t240: F) -> (F,) {
    let t120890 = -t119875 + t119877 - t119879 + t119881 - t119883 + t119885 - t119886 + t240 * (t119901 + t119998 + t120849 + t120886) + t120006 - t120829 + t120836 + t120841 - t120844 + t120851 - t120854 - t120855 + t120857 - t120877 - t120883;
    (t120890,)
}
