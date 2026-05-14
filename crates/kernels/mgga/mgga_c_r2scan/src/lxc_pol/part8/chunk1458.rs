//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1458/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1458<F: Float>(t1044: F, t19341: F, t19388: F, t19394: F, t23798: F, t32219: F, t32222: F, t32225: F, t32228: F, t32269: F, t34819: F, t9782: F, t1039: F, t19405: F, t23829: F, t32963: F, t32965: F, t32967: F, t32968: F, t34822: F, t34827: F, t34830: F, t34833: F, t9832: F) -> (F, F) {
    let t35258 = 3.0 * t1044 * t9782 - t19341 - t19388 - t19394 + t23798 - t32219 + t32222 - t32225 - t32228 + t32269 + t34819;
    let t35264 = 3.0 * t1039 * t9832 + t19405 + t23829 - t32963 + t32965 - t32967 - t32968 + t34822 + t34827 - t34830 - t34833;
    (t35258, t35264)
}
