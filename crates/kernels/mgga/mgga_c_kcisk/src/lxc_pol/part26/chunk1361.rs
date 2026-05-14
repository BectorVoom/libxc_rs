//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1361/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1361<F: Float>(t5886: F, t6340: F, t6336: F, t119833: F, t119835: F, t119837: F, t119839: F, t119841: F, t119843: F, t119845: F, t119847: F, t119849: F, t119852: F, t119854: F, t119856: F, t119858: F, t119860: F, t119862: F, t119864: F, t119866: F) -> (F, F, F) {
    let t119868 = t5886 * t6340;
    let t119870 = t5886 * t6336;
    let t119872 = t119833 / 48.0 + t119835 / 24.0 - t119837 / 16.0 + t119839 / 8.0 - t119841 / 3.0 - t119843 / 12.0 + t119845 / 54.0 + t119847 / 3.0 - t119849 / 64.0 - t119852 / 16.0 + t119854 / 128.0 + t119856 / 4.0 + t119858 / 12.0 + 3.0 / 64.0 * t119860 + t119862 / 72.0 + t119864 / 8.0 - t119866 / 96.0 + t119868 / 64.0 - t119870 / 12.0;
    (t119868, t119870, t119872)
}
