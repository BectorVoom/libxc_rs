//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 721/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk721(t5686: f64, t650: f64, t653: f64, t685: f64, t63: f64, t688: f64, t206: f64, t1399: f64, t1823: f64, t1842: f64, t1933: f64, t1939: f64, t1957: f64, t208: f64, t220: f64, t390: f64, t5589: f64, t5658: f64, t5661: f64, t5664: f64, t5669: f64, t5678: f64, t5682: f64, t718: f64) -> (f64, f64) {
    let t5689 = 0.16081979498692535067e2_f64 * t650 * t653 * t5686;
    let t5693 = t685 * t685;
    let t5694 = 1.0_f64 / t5693;
    let t5695 = t63 * t5694;
    let t5696 = t688 * t688;
    let t5697 = 1.0_f64 / t5696;
    let t5698 = t206 * t5697;
    let t5702 = -0.28518989949414381017e2_f64 * t390 * t1823 - 0.96319466275353142157e0_f64 * t390 * t1842 - 0.13698666666666666666e0_f64 * t1399 * t1933 + 0.22030167649275614036e1_f64 * t1399 * t1939 + 0.5848223622634646207e0_f64 * t220 * t5658 + 0.51947577317044391277e2_f64 * t718 * t5661 + 0.17315859105681463759e2_f64 * t718 * t5664 - t5669 - t5678 - t5682 - t5689 - 24.0_f64 * t1957 * t208 * t5589 + 0.19964560303604640732e6_f64 * t5695 * t5698 * t5589;
    (t5689, t5702)
}
