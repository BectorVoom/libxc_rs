//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1254/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1254<F: Float>(t15691: F, t19985: F, t1011: F, t1068: F, t15689: F, t15700: F, t19951: F, t19954: F, t19957: F, t19960: F, t19963: F, t19968: F, t19973: F, t19977: F, t19982: F, t3106: F, t4892: F, t6331: F) -> F {
    let t19986 = t15691 * t19985;
    let t19989 = t1011 * t19951 / F::cast_from(216.0_f64) + t1011 * t19954 / F::cast_from(108.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1011 * t19957 + t1011 * t19960 / F::cast_from(48.0_f64) - t1011 * t19963 / F::cast_from(72.0_f64) + F::cast_from(0.15244095330869239812e-2_f64) * t3106 * t6331 + F::cast_from(0.14291339372689912324e-3_f64) * t19968 * t1068 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t19973 - F::cast_from(0.28582678745379824648e-3_f64) * t19977 + F::cast_from(0.47637797908966374413e-3_f64) * t15700 * t19982 - F::cast_from(0.28582678745379824648e-3_f64) * t15689 * t19986;
    t19989
}
