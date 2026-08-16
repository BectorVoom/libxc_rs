//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2990/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990<F: Float>(t4772: F, t6305: F, t11675: F, t11774: F, t11875: F, t11927: F, t15696: F, t19730: F, t19777: F, t23852: F, t23908: F, t3117: F, t3162: F, t3188: F, t42326: F, t53613: F, t6271: F, t65859: F, t65892: F, t65894: F, t65931: F, t65960: F, t65965: F, t66003: F, t66017: F, t66022: F, t66024: F, t66029: F) -> (F, F) {
    let t79275 = t4772 * t6305;
    let t79287 = F::cast_from(0.63517063878621832551e-4_f64) * t42326 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15696 * t19730 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15696 * t19777 + t53613 - F::cast_from(0.19055119163586549765e-3_f64) * t65859 - F::cast_from(0.30488190661738479624e-2_f64) * t65892 - F::cast_from(0.57165357490759649295e-3_f64) * t65931 - F::cast_from(0.45732285992607719437e-2_f64) * t65960 + F::cast_from(0.1270341277572436651e-2_f64) * t65965 + F::cast_from(0.25724410870841842184e-2_f64) * t11927 * t3117 * t6271 * t65894 + F::cast_from(0.42874018118069736972e-3_f64) * t66003 + F::cast_from(0.42874018118069736972e-3_f64) * t66017 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t79275 * t3162 - F::cast_from(0.14291339372689912324e-3_f64) * t66022 + F::cast_from(0.22866142996303859718e-2_f64) * t66024 - F::cast_from(0.85748036236139473944e-3_f64) * t3188 * t23852 - F::cast_from(0.28582678745379824648e-3_f64) * t66029 + F::cast_from(0.42874018118069736972e-3_f64) * t11675 * t23908;
    (t79275, t79287)
}
