//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1164/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1164<F: Float>(t35951: F, t2001: F, t5232: F, t2327: F, t7610: F, t31676: F, t31680: F, t31682: F, t31684: F, t31687: F, t35924: F, t35927: F, t35928: F, t35931: F, t35935: F, t35936: F, t35938: F, t35942: F, t35947: F, t35949: F) -> F {
    let t35952 = F::cast_from(0.17149607247227894789e-2_f64) * t35951;
    let t35953 = t2001 * t5232;
    let t35955 = t7610 * t2327;
    let t35957 = F::cast_from(13.0_f64) / F::cast_from(288.0_f64) * t35924 + t35927 - F::cast_from(0.34299214494455789578e-2_f64) * t35928 - t35931 - t35935 - F::cast_from(0.19865625e0_f64) * t35936 - F::cast_from(0.1324375e0_f64) * t35938 - F::cast_from(0.34299214494455789578e-2_f64) * t31676 + F::cast_from(0.85748036236139473944e-3_f64) * t31680 - F::cast_from(0.17149607247227894789e-2_f64) * t35942 + F::cast_from(0.55907719625962937008e-2_f64) * t31682 - F::cast_from(0.62896184579208304136e-3_f64) * t31684 - F::cast_from(0.14291339372689912324e-3_f64) * t31687 + F::cast_from(0.85748036236139473944e-3_f64) * t35947 - F::cast_from(0.85748036236139473944e-3_f64) * t35949 - t35952 - F::cast_from(0.85748036236139473944e-3_f64) * t35953 + F::cast_from(0.10718504529517434243e-3_f64) * t35955;
    t35957
}
