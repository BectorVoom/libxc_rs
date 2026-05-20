//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1581/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1581<F: Float>(t11264: F, t11675: F, t11818: F, t11859: F, t11875: F, t11927: F, t15583: F, t15618: F, t15662: F, t15707: F, t15862: F, t15865: F, t15892: F, t15926: F, t15942: F, t19622: F, t19626: F, t19636: F, t19641: F, t19645: F, t19685: F, t19729: F, t19763: F, t19797: F, t19813: F, t19841: F, t19885: F, t19895: F, t19901: F, t19923: F, t19950: F, t19989: F, t20012: F, t20036: F, t20073: F, t20108: F, t3091: F, t3127: F, t3241: F, t4783: F, t4825: F, t4899: F, t4907: F, t6268: F, t6285: F) -> F {
    let t20112 = -F::cast_from(0.14291339372689912324e-3_f64) * t4899 * t19626 + F::cast_from(0.28582678745379824648e-3_f64) * t15618 * t4783 + F::cast_from(0.28582678745379824648e-3_f64) * t11675 * t6268 - F::cast_from(0.85748036236139473944e-3_f64) * t11859 * t19636 + F::cast_from(0.42874018118069736972e-3_f64) * t11875 * t19641 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t19645 - F::cast_from(0.42874018118069736972e-3_f64) * t15926 * t4907 - F::cast_from(0.28582678745379824648e-3_f64) * t15707 * t4825 + F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t19895 + t3241 * t6285 / F::new(54.0) + t20036 - t19901 / F::new(432.0) + t20012 + t19923 + F::cast_from(0.47637797908966374413e-4_f64) * t11818 + t19729 + t19885 + t20073 + t19989 + t19797 + t19813 + t19685 + t20108 - t15892 - t15583 + t15942 - F::cast_from(0.95275595817932748827e-4_f64) * t15862 + t15865 + t19841 + t19763 - F::cast_from(0.47637797908966374413e-4_f64) * t11264 + t19950 - t15662 + F::cast_from(0.85748036236139473944e-3_f64) * t11927 * t19622;
    t20112
}
