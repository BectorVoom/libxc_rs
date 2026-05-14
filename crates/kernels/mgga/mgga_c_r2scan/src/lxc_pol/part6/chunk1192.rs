//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1192/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1192<F: Float>(t1691: F, t1945: F, t2006: F, t208: F, t21416: F, t21467: F, t21874: F, t21884: F, t21887: F, t21899: F, t21902: F, t21914: F, t5269: F, t5270: F, t5317: F, t5486: F, t5490: F, t5537: F, t5542: F, t5658: F, t625: F, t682: F, t687: F, t690: F, t699: F, t705: F, t712: F, t713: F) -> (F,) {
    let t21918 = -0.62337092780453269531e3 * t1945 * t5542 * t1691 + t21874 + 0.36433041676861022416e6 * t5537 * t712 * t5269 * t5270 - 0.46785788981077169656e1 * t705 * t713 * t5317 - t21884 - t21887 - 0.21309037037037037036e0 * t625 * t5490 * t682 - 0.67471172535210825684e-1 * t625 * t5486 * t713 - 0.21687162600603479684e-1 * t625 * t699 * t5658 + t21899 + t21902 + 0.32163958997385070134e2 * t687 * t690 * t21467 - t21914 + 120.0 * t2006 * t208 * t21416;
    (t21918,)
}
