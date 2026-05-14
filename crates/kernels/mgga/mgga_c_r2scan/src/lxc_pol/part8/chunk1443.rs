//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1443/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1443<F: Float>(t10300: F, t2271: F, t10090: F, t2267: F, t2858: F, t19709: F, t25032: F, t32994: F, t32995: F, t32996: F, t32997: F, t32998: F, t33746: F, t33749: F, t34887: F, t881: F) -> (F, F) {
    let t34888 = t2271 * t10300;
    let t34896 = 18.0 * t2858 * t2267 * t10090;
    let t34897 = t32994 + t34887 - 0.7089e1 * t34888 - 0.7089e1 * t881 * t33746 - 0.7089e1 * t881 * t33749 + t25032 + t32995 + t32996 + t32997 - t19709 - t34896 - t32998;
    (t34896, t34897)
}
