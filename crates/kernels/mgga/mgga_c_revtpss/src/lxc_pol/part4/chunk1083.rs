//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1083/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1083<F: Float>(t4416: F, t808: F, t10886: F, t2703: F, t4458: F, t10678: F, t10682: F, t10687: F, t10692: F, t14759: F, t14761: F, t14765: F, t14769: F, t14774: F, t14777: F, t851: F) -> (F,) {
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14783 = 7.0 / 72.0 * t2703 * t4458;
    let t14784 = t14759 - 0.45178982497454656791e-5 * t14761 - 0.60976381323476959249e-3 * t10678 + 0.28582678745379824648e-4 * t10682 - t10687 + t10692 - 35.0 / 216.0 * t14765 + 0.42874018118069736972e-2 * t851 * t14769 - 0.25724410870841842183e-1 * t851 * t14774 - 0.80031500487063509015e-2 * t14777 + 0.10164000561857065645e-4 * t14780 + t14783;
    (t14784,)
}
