//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1297/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1297<F: Float>(t1011: F, t1063: F, t11656: F, t11994: F, t11999: F, t16057: F, t16062: F, t16064: F, t19930: F, t19934: F, t19940: F, t19944: F, t19947: F, t3127: F, t4837: F, t6263: F, t6312: F) -> F {
    let t19950 = F::cast_from(0.15244095330869239812e-2_f64) * t11656 * t6263 + F::cast_from(0.11433071498151929859e-2_f64) * t11999 * t6312 + F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t19930 - F::cast_from(0.57165357490759649296e-3_f64) * t1063 * t19934 - F::cast_from(0.28582678745379824648e-3_f64) * t11994 * t6263 - F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t19940 + t16057 + t16062 - t16064 + F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t19944 - t1011 * t19947 / F::cast_from(144.0_f64);
    t19950
}
