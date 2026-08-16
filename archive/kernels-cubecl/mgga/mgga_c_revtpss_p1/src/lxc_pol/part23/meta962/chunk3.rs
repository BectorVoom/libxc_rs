//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3253/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3253<F: Float>(t1868: F, t5658: F, t6843: F, t9994: F, t6869: F, t73731: F, t9816: F, t9818: F, t22829: F, t9962: F, t1882: F, t13783: F, t13784: F, t13789: F, t13804: F, t13926: F, t1399: F, t1883: F, t23037: F, t36776: F, t3934: F, t3938: F, t5671: F, t5673: F, t5675: F, t73803: F, t73805: F, t73811: F, t73813: F, t73815: F, t73818: F, t73842: F, t74700: F, t85553: F, t85585: F) -> (F, F, F, F) {
    let t85625 = t1868 * t5658;
    let t85638 = t6843 * t9994;
    let t85648 = t9816 * t9818 * t73731 * t6869;
    let t85652 = t9962 * t22829;
    let t85659 = t6843 * t1882;
    let t85680 = F::cast_from(0.51448821741683684366e-2_f64) * t3934 * t13789 * t1883 * t85625 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t73842 * t6869 - F::cast_from(0.51448821741683684366e-2_f64) * t5671 * t13789 * t23037 * t13784 - F::cast_from(0.38586616306262763276e-2_f64) * t13804 * t36776 * t85638 * t13926 + F::cast_from(0.60023625365297631762e-2_f64) * t73803 + F::cast_from(0.30011812682648815881e-2_f64) * t73805 - F::cast_from(0.18007087609589289528e-1_f64) * t73811 + F::cast_from(0.15246000842785598467e-3_f64) * t85648 + F::cast_from(0.24009450146119052705e-1_f64) * t73813 - F::cast_from(0.12004725073059526352e-1_f64) * t73815 - F::cast_from(0.12004725073059526352e-1_f64) * t85652 - F::cast_from(0.12004725073059526352e-1_f64) * t73818 + F::cast_from(0.30011812682648815881e-2_f64) * t5671 * t5673 * t85553 * t5675 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t85659 * t3938 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t36776 * t85659 * t1399 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t85585 * t3938 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t74700 * t6869 + F::cast_from(0.38586616306262763276e-2_f64) * t5671 * t36776 * t85659 * t5675;
    (t85625, t85638, t85659, t85680)
}
