//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 711/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk711<F: Float>(t5686: F, t650: F, t653: F, t685: F, t63: F, t688: F, t206: F, t1399: F, t1823: F, t1842: F, t1933: F, t1939: F, t1957: F, t208: F, t220: F, t390: F, t5589: F, t5658: F, t5661: F, t5664: F, t5669: F, t5678: F, t5682: F, t718: F) -> (F, F) {
    let t5689 = F::cast_from(0.16081979498692535067e2_f64) * t650 * t653 * t5686;
    let t5693 = t685 * t685;
    let t5694 = F::cast_from(1.0_f64) / t5693;
    let t5695 = t63 * t5694;
    let t5696 = t688 * t688;
    let t5697 = F::cast_from(1.0_f64) / t5696;
    let t5698 = t206 * t5697;
    let t5702 = -F::cast_from(0.28518989949414381017e2_f64) * t390 * t1823 - F::cast_from(0.96319466275353142157e0_f64) * t390 * t1842 - F::cast_from(0.13698666666666666666e0_f64) * t1399 * t1933 + F::cast_from(0.22030167649275614036e1_f64) * t1399 * t1939 + F::cast_from(0.5848223622634646207e0_f64) * t220 * t5658 + F::cast_from(0.51947577317044391277e2_f64) * t718 * t5661 + F::cast_from(0.17315859105681463759e2_f64) * t718 * t5664 - t5669 - t5678 - t5682 - t5689 - F::cast_from(24.0_f64) * t1957 * t208 * t5589 + F::cast_from(0.19964560303604640732e6_f64) * t5695 * t5698 * t5589;
    (t5689, t5702)
}
