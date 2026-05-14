//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1067/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1067<F: Float>(t19885: F, t3442: F, t1176: F, t6682: F, t1133: F, t6491: F, t5077: F, t3337: F, t19847: F, t19850: F, t19852: F, t19854: F, t19858: F, t19860: F, t19863: F, t19866: F, t19868: F, t19871: F, t19873: F, t19875: F, t19877: F, t19880: F, t19883: F) -> (F, F, F, F, F) {
    let t19886 = t3442 * t19885;
    let t19888 = t6682 * t1176;
    let t19890 = t6491 * t1133;
    let t19891 = t5077 * t19890;
    let t19892 = t3337 * t19891;
    let t19894 = -t19847 / 288.0 + t19850 / 96.0 + t19852 / 48.0 + 2.0 / 9.0 * t19854 - t19858 / 48.0 - t19860 / 12.0 + t19863 / 36.0 - t19866 / 128.0 + t19868 / 24.0 - t19871 / 24.0 - t19873 / 12.0 + t19875 / 3.0 + t19877 / 96.0 - t19880 / 72.0 + 3.0 / 128.0 * t19883 + t19886 / 24.0 - t19888 / 6.0 + t19892 / 36.0;
    (t19886, t19888, t19890, t19892, t19894)
}
