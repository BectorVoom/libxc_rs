//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1229/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1229(t2003: f64, t2888: f64, t2916: f64, t300: f64, t779: f64, t5728: f64, t17797: f64, t17814: f64, t2031: f64, t2104: f64, t2105: f64, t2887: f64, t2899: f64, t2922: f64, t2945: f64, t302: f64, t5568: f64, t5694: f64, t5699: f64, t5966: f64, t5979: f64, t655: f64, t7350: f64, t7391: f64, t758: f64, t759: f64, t761: f64, t7640: f64, t7664: f64, t7666: f64, t7695: f64, t7701: f64, t7702: f64, t7796: f64, t9258: f64) -> (f64, f64, f64) {
    let t21395 = t2888 * t2003;
    let t21417 = t300 * t779 * t2916;
    let t21435 = t2916 * t5728;
    let t21442 = -3.0_f64 / 16.0_f64 * t2887 * t21395 * t7391 * t655 - 0.15434646522505105311e-1_f64 * t2945 * t758 * t7796 * t5568 + 0.38586616306262763276e-2_f64 * t2104 * t9258 * t761 * t5568 + 0.77173232612525526551e-2_f64 * t2899 * t9258 * t2031 * t5694 - 0.38586616306262763275e-2_f64 * t2922 * t9258 * t7701 * t7640 + 0.25724410870841842183e-2_f64 * t2922 * t21417 * t7702 + 0.38586616306262763276e-2_f64 * t2104 * t7695 * t5699 + 0.38586616306262763276e-2_f64 * t2104 * t7695 * t5979 + 0.77173232612525526551e-2_f64 * t2899 * t7695 * t5966 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t7350 * t759 * t761 + 0.64311027177104605458e-3_f64 * t7664 * t302 * t21435 * t7666 - 0.85748036236139473944e-3_f64 * t17797 + 0.85748036236139473944e-3_f64 * t17814;
    (t21417, t21435, t21442)
}
