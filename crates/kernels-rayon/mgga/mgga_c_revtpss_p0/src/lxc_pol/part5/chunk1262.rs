//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1262/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1262(t11264: f64, t11675: f64, t11818: f64, t11859: f64, t11875: f64, t11927: f64, t15583: f64, t15618: f64, t15662: f64, t15707: f64, t15862: f64, t15865: f64, t15892: f64, t15926: f64, t15942: f64, t19622: f64, t19626: f64, t19636: f64, t19641: f64, t19645: f64, t19685: f64, t19729: f64, t19763: f64, t19797: f64, t19813: f64, t19841: f64, t19885: f64, t19895: f64, t19901: f64, t19923: f64, t19950: f64, t19989: f64, t20012: f64, t20036: f64, t20073: f64, t20108: f64, t3091: f64, t3127: f64, t3241: f64, t4783: f64, t4825: f64, t4899: f64, t4907: f64, t6268: f64, t6285: f64) -> f64 {
    let t20112 = -0.14291339372689912324e-3_f64 * t4899 * t19626 + 0.28582678745379824648e-3_f64 * t15618 * t4783 + 0.28582678745379824648e-3_f64 * t11675 * t6268 - 0.85748036236139473944e-3_f64 * t11859 * t19636 + 0.42874018118069736972e-3_f64 * t11875 * t19641 + 0.14291339372689912324e-3_f64 * t3091 * t19645 - 0.42874018118069736972e-3_f64 * t15926 * t4907 - 0.28582678745379824648e-3_f64 * t15707 * t4825 + 0.28582678745379824648e-3_f64 * t3127 * t19895 + t3241 * t6285 / 54.0_f64 + t20036 - t19901 / 432.0_f64 + t20012 + t19923 + 0.47637797908966374413e-4_f64 * t11818 + t19729 + t19885 + t20073 + t19989 + t19797 + t19813 + t19685 + t20108 - t15892 - t15583 + t15942 - 0.95275595817932748827e-4_f64 * t15862 + t15865 + t19841 + t19763 - 0.47637797908966374413e-4_f64 * t11264 + t19950 - t15662 + 0.85748036236139473944e-3_f64 * t11927 * t19622;
    t20112
}
