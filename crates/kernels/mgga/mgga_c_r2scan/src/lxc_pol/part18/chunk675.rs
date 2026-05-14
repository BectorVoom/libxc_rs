//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 675/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk675<F: Float>(t5696: F, t206: F, t1399: F, t1823: F, t1842: F, t1933: F, t1939: F, t1957: F, t208: F, t220: F, t390: F, t5589: F, t5658: F, t5661: F, t5664: F, t5669: F, t5678: F, t5682: F, t5689: F, t5695: F, t718: F) -> (F,) {
    let t5697 = 1.0 / t5696;
    let t5698 = t206 * t5697;
    let t5702 = -0.28518989949414381017e2 * t390 * t1823 - 0.96319466275353142157e0 * t390 * t1842 - 0.13698666666666666666e0 * t1399 * t1933 + 0.22030167649275614036e1 * t1399 * t1939 + 0.5848223622634646207e0 * t220 * t5658 + 0.51947577317044391277e2 * t718 * t5661 + 0.17315859105681463759e2 * t718 * t5664 - t5669 - t5678 - t5682 - t5689 - 24.0 * t1957 * t208 * t5589 + 0.19964560303604640732e6 * t5695 * t5698 * t5589;
    (t5702,)
}
