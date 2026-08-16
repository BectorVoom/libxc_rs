//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1430/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1430<F: Float>(t1260: F, t5326: F, t17376: F, t3599: F, t17482: F, t3604: F, t3720: F, t3372: F, t5277: F, t1042: F, t12855: F, t12964: F, t12979: F, t12985: F, t12996: F, t3606: F, t3620: F, t3640: F, t3711: F, t3714: F, t5381: F, t5391: F) -> F {
    let t17569 = t5326 * t1260;
    let t17572 = t17376 * t3599;
    let t17579 = t17482 * t3604;
    let t17580 = t3720 * t17579;
    let t17583 = t5277 * t3372;
    let t17584 = t1042 * t17583;
    let t17587 = F::cast_from(0.7622047665434619906e-3_f64) * t5391 * t3640 + F::cast_from(0.23818898954483187207e-3_f64) * t5381 * t3620 - F::cast_from(0.1270341277572436651e-2_f64) * t5391 * t3620 + F::cast_from(0.28582678745379824648e-3_f64) * t17569 * t3714 + F::cast_from(0.42874018118069736972e-3_f64) * t17572 * t3606 - F::cast_from(0.14291339372689912324e-3_f64) * t12964 - F::cast_from(0.28582678745379824648e-3_f64) * t12979 + F::cast_from(0.95275595817932748826e-4_f64) * t12985 + F::cast_from(0.28582678745379824648e-3_f64) * t12996 - F::cast_from(0.42874018118069736972e-3_f64) * t12855 * t17580 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t17584;
    t17587
}
