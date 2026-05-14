//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 788/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk788<F: Float>(t1901: F, t24731: F, t24733: F, t24735: F, t24739: F, t24742: F, t24744: F, t24749: F, t24753: F, t24757: F, t24758: F, t24761: F, t24765: F, t24770: F, t24775: F, t24778: F, t24781: F, t24785: F, t446: F) -> (F,) {
    let t24788 = 2.0 / 9.0 * t24731 + 2.0 / 9.0 * t24733 + 2.0 / 9.0 * t24735 - 4.0 / 3.0 * t1901 * t24739 - 2.0 / 27.0 * t24742 + 2.0 / 9.0 * t1901 * t24744 + 2.0 / 9.0 * t1901 * t24749 + t1901 * t24753 / 9.0 - t24757 + 2.0 / 9.0 * t24758 - 4.0 / 9.0 * t1901 * t24761 + 2.0 / 3.0 * t446 * t24765 + 2.0 / 3.0 * t446 * t24770 + t446 * t24775 / 3.0 + 4.0 / 3.0 * t446 * t24778 - t446 * t24781 / 3.0 - 2.0 / 3.0 * t446 * t24785;
    (t24788,)
}
