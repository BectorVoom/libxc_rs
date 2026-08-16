//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3499/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3499(t12013: f64, t20029: f64, t1063: f64, t19671: f64, t3172: f64, t1042: f64, t16186: f64, t16199: f64, t16208: f64, t19663: f64, t19672: f64, t3127: f64, t3188: f64, t4801: f64, t4879: f64, t53473: f64, t54537: f64, t60834: f64, t65365: f64, t65370: f64, t65829: f64, t65931: f64, t65947: f64) -> f64 {
    let t65960 = t12013 * t20029;
    let t65965 = t1063 * t3172 * t19671;
    let t65973 = -0.3811023832717309953e-3_f64 * t65931 - 0.28582678745379824648e-3_f64 * t1063 * t1042 * t4801 * t60834 + 0.1270341277572436651e-2_f64 * t3188 * t19672 + 0.63517063878621832552e-3_f64 * t1063 * t1042 * t16208 * t65829 + 0.28582678745379824648e-3_f64 * t3127 * t1042 * t4801 * t65365 + 0.23289590088828005269e-2_f64 * t1063 * t1042 * t53473 * t65947 - 0.14291339372689912324e-2_f64 * t1063 * t1042 * t16199 * t65829 - 0.76220476654346199062e-2_f64 * t1063 * t1042 * t54537 * t65947 - 0.30488190661738479624e-2_f64 * t65960 - 0.28582678745379824648e-2_f64 * t3188 * t19663 + 0.8468941850482911007e-3_f64 * t65965 + 0.42874018118069736972e-3_f64 * t4879 * t16186 - 0.14291339372689912324e-2_f64 * t1063 * t1042 * t16199 * t65370;
    t65973
}
