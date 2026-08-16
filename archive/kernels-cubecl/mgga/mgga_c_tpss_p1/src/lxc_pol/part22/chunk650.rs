//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 650/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk650<F: Float>(t2975: F, t3001: F, t1054: F, t1063: F, t1073: F, t1082: F, t2856: F, t2859: F, t2866: F, t2908: F, t2916: F, t2922: F, t2925: F, t2930: F, t2932: F, t2950: F, t2955: F, t2958: F, t2967: F, t2969: F, t2974: F, t2976: F, t2994: F, t2999: F, t421: F) -> (F, F) {
    let t3002 = t2975 * t3001;
    let t3005 = -F::cast_from(0.310907e-1_f64) * t2922 * t421 + F::cast_from(2.0_f64) * t2925 * t1063 - F::cast_from(2.0_f64) * t2930 * t2932 + F::cast_from(1.0_f64) * t1054 * t2950 + F::cast_from(0.32163958997385070134e2_f64) * t2955 * t2958 + t2856 - t2859 + t2866 - t2908 - t2916 - F::cast_from(0.19751673498613801407e-1_f64) * t2967 + F::cast_from(0.11696447245269292414e1_f64) * t2969 * t1082 - F::cast_from(0.11696447245269292414e1_f64) * t2974 * t2976 + F::cast_from(0.5848223622634646207e0_f64) * t1073 * t2994 + F::cast_from(0.17315859105681463759e2_f64) * t2999 * t3002;
    (t3002, t3005)
}
