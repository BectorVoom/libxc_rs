//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2990/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990(t4772: f64, t6305: f64, t11675: f64, t11774: f64, t11875: f64, t11927: f64, t15696: f64, t19730: f64, t19777: f64, t23852: f64, t23908: f64, t3117: f64, t3162: f64, t3188: f64, t42326: f64, t53613: f64, t6271: f64, t65859: f64, t65892: f64, t65894: f64, t65931: f64, t65960: f64, t65965: f64, t66003: f64, t66017: f64, t66022: f64, t66024: f64, t66029: f64) -> (f64, f64) {
    let t79275 = t4772 * t6305;
    let t79287 = 0.63517063878621832551e-4_f64 * t42326 - 0.85748036236139473944e-3_f64 * t11774 * t15696 * t19730 - 0.85748036236139473944e-3_f64 * t11774 * t15696 * t19777 + t53613 - 0.19055119163586549765e-3_f64 * t65859 - 0.30488190661738479624e-2_f64 * t65892 - 0.57165357490759649295e-3_f64 * t65931 - 0.45732285992607719437e-2_f64 * t65960 + 0.1270341277572436651e-2_f64 * t65965 + 0.25724410870841842184e-2_f64 * t11927 * t3117 * t6271 * t65894 + 0.42874018118069736972e-3_f64 * t66003 + 0.42874018118069736972e-3_f64 * t66017 + 0.64311027177104605458e-3_f64 * t11875 * t3117 * t79275 * t3162 - 0.14291339372689912324e-3_f64 * t66022 + 0.22866142996303859718e-2_f64 * t66024 - 0.85748036236139473944e-3_f64 * t3188 * t23852 - 0.28582678745379824648e-3_f64 * t66029 + 0.42874018118069736972e-3_f64 * t11675 * t23908;
    (t79275, t79287)
}
