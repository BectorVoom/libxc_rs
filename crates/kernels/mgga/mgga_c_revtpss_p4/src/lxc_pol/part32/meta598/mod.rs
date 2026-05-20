//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta598<F: Float>(t2110: F, t5808: F, t1455: F, t8130: F, t1921: F, t7541: F, t28944: F, t575: F, t5891: F, t94978: F, t665: F, t94982: F, t1513: F, t4287: F, t25826: F, t25823: F, t5915: F, t21876: F, t6998: F, t28166: F, t7897: F, t5824: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104079, t104081, t104083, t104085, t105870, t105873) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932::<F>(t2110, t5808, t1455, t8130, t1921, t7541, t28944, t575, t5891, t94978, t665, t94982);
        let (t105876, t105878, t105881, t105883, t105892, t105898) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933::<F>(t1513, t4287, t25826, t25823, t5915, t665, t21876, t6998, t28166, t7897, t5824, t775);
    (t104079, t104081, t104083, t104085, t105870, t105873, t105876, t105878, t105881, t105883, t105892, t105898)
}
