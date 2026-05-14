//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1348/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1348<F: Float>(t28069: F, t23953: F, t19743: F, t19624: F, t19628: F, t19646: F, t19649: F, t19687: F, t19694: F, t19698: F, t19728: F, t19748: F, t23951: F, t23956: F, t23961: F, t23982: F) -> (F, F, F, F) {
    let t32990 = 0.54934341918019635162e-3 * t28069;
    let t32991 = 0.15584273195113317383e3 * t23953;
    let t32992 = 0.5848223622634646207e0 * t19743;
    let t32993 = t19624 - t19628 - t19646 - t19649 - t19728 + t19687 + t23951 - t32990 - t32991 + t23956 + t23961 - t32992 - t19748 + t23982 - t19694 + t19698;
    (t32990, t32991, t32992, t32993)
}
