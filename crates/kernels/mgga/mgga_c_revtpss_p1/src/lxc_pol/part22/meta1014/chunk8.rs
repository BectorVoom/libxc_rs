//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3499/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3499<F: Float>(t12013: F, t20029: F, t1063: F, t19671: F, t3172: F, t1042: F, t16186: F, t16199: F, t16208: F, t19663: F, t19672: F, t3127: F, t3188: F, t4801: F, t4879: F, t53473: F, t54537: F, t60834: F, t65365: F, t65370: F, t65829: F, t65931: F, t65947: F) -> F {
    let t65960 = t12013 * t20029;
    let t65965 = t1063 * t3172 * t19671;
    let t65973 = -F::cast_from(0.3811023832717309953e-3_f64) * t65931 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t1042 * t4801 * t60834 + F::cast_from(0.1270341277572436651e-2_f64) * t3188 * t19672 + F::cast_from(0.63517063878621832552e-3_f64) * t1063 * t1042 * t16208 * t65829 + F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t4801 * t65365 + F::cast_from(0.23289590088828005269e-2_f64) * t1063 * t1042 * t53473 * t65947 - F::cast_from(0.14291339372689912324e-2_f64) * t1063 * t1042 * t16199 * t65829 - F::cast_from(0.76220476654346199062e-2_f64) * t1063 * t1042 * t54537 * t65947 - F::cast_from(0.30488190661738479624e-2_f64) * t65960 - F::cast_from(0.28582678745379824648e-2_f64) * t3188 * t19663 + F::cast_from(0.8468941850482911007e-3_f64) * t65965 + F::cast_from(0.42874018118069736972e-3_f64) * t4879 * t16186 - F::cast_from(0.14291339372689912324e-2_f64) * t1063 * t1042 * t16199 * t65370;
    t65973
}
