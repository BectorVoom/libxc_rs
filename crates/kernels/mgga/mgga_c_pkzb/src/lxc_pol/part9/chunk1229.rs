//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1229/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1229<F: Float>(t2003: F, t2888: F, t2916: F, t300: F, t779: F, t5728: F, t17797: F, t17814: F, t2031: F, t2104: F, t2105: F, t2887: F, t2899: F, t2922: F, t2945: F, t302: F, t5568: F, t5694: F, t5699: F, t5966: F, t5979: F, t655: F, t7350: F, t7391: F, t758: F, t759: F, t761: F, t7640: F, t7664: F, t7666: F, t7695: F, t7701: F, t7702: F, t7796: F, t9258: F) -> (F, F, F) {
    let t21395 = t2888 * t2003;
    let t21417 = t300 * t779 * t2916;
    let t21435 = t2916 * t5728;
    let t21442 = -F::new(3.0) / F::new(16.0) * t2887 * t21395 * t7391 * t655 - F::cast_from(0.15434646522505105311e-1_f64) * t2945 * t758 * t7796 * t5568 + F::cast_from(0.38586616306262763276e-2_f64) * t2104 * t9258 * t761 * t5568 + F::cast_from(0.77173232612525526551e-2_f64) * t2899 * t9258 * t2031 * t5694 - F::cast_from(0.38586616306262763275e-2_f64) * t2922 * t9258 * t7701 * t7640 + F::cast_from(0.25724410870841842183e-2_f64) * t2922 * t21417 * t7702 + F::cast_from(0.38586616306262763276e-2_f64) * t2104 * t7695 * t5699 + F::cast_from(0.38586616306262763276e-2_f64) * t2104 * t7695 * t5979 + F::cast_from(0.77173232612525526551e-2_f64) * t2899 * t7695 * t5966 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t2105 * t7350 * t759 * t761 + F::cast_from(0.64311027177104605458e-3_f64) * t7664 * t302 * t21435 * t7666 - F::cast_from(0.85748036236139473944e-3_f64) * t17797 + F::cast_from(0.85748036236139473944e-3_f64) * t17814;
    (t21417, t21435, t21442)
}
