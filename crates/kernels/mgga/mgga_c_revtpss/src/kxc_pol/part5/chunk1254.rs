//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1254/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1254<F: Float>(t15691: F, t19985: F, t1011: F, t1068: F, t15689: F, t15700: F, t19951: F, t19954: F, t19957: F, t19960: F, t19963: F, t19968: F, t19973: F, t19977: F, t19982: F, t3106: F, t4892: F, t6331: F) -> F {
    let t19986 = t15691 * t19985;
    let t19989 = t1011 * t19951 / F::new(216.0) + t1011 * t19954 / F::new(108.0) + F::new(7.0) / F::new(648.0) * t1011 * t19957 + t1011 * t19960 / F::new(48.0) - t1011 * t19963 / F::new(72.0) + F::new(0.15244095330869239812e-2) * t3106 * t6331 + F::new(0.14291339372689912324e-3) * t19968 * t1068 + F::new(0.85748036236139473944e-3) * t4892 * t19973 - F::new(0.28582678745379824648e-3) * t19977 + F::new(0.47637797908966374413e-3) * t15700 * t19982 - F::new(0.28582678745379824648e-3) * t15689 * t19986;
    t19989
}
