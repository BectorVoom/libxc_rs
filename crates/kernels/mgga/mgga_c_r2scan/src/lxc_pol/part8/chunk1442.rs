//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1442/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1442<F: Float>(t19694: F, t19698: F, t19702: F, t19748: F, t23956: F, t23961: F, t23982: F, t23986: F, t31459: F, t32990: F, t32991: F, t32992: F, t34881: F, t2266: F, t2867: F, t3016: F, t795: F) -> (F, F) {
    let t34882 = 3.0 * t31459 + t32990 + t34881 + t32991 - t23956 - t23961 + t32992 + t19748 - t23982 + t19694 - t19698 + t23986 - t19702;
    let t34887 = 9.0 * t2266 * t2867 * t3016 * t795;
    (t34882, t34887)
}
