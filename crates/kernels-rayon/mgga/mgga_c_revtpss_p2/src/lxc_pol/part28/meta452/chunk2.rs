//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1712/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1712(t1260: f64, t5326: f64, t17376: f64, t3599: f64, t17482: f64, t3604: f64, t3720: f64, t3372: f64, t5277: f64, t1042: f64, t12855: f64, t12964: f64, t12979: f64, t12985: f64, t12996: f64, t3606: f64, t3620: f64, t3640: f64, t3711: f64, t3714: f64, t5381: f64, t5391: f64) -> f64 {
    let t17569 = t5326 * t1260;
    let t17572 = t17376 * t3599;
    let t17579 = t17482 * t3604;
    let t17580 = t3720 * t17579;
    let t17583 = t5277 * t3372;
    let t17584 = t1042 * t17583;
    let t17587 = 0.7622047665434619906e-3_f64 * t5391 * t3640 + 0.23818898954483187207e-3_f64 * t5381 * t3620 - 0.1270341277572436651e-2_f64 * t5391 * t3620 + 0.28582678745379824648e-3_f64 * t17569 * t3714 + 0.42874018118069736972e-3_f64 * t17572 * t3606 - 0.14291339372689912324e-3_f64 * t12964 - 0.28582678745379824648e-3_f64 * t12979 + 0.95275595817932748826e-4_f64 * t12985 + 0.28582678745379824648e-3_f64 * t12996 - 0.42874018118069736972e-3_f64 * t12855 * t17580 + 0.14291339372689912324e-3_f64 * t3711 * t17584;
    t17587
}
