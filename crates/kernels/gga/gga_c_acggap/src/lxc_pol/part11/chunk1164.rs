//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1164/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1164<F: Float>(t35951: F, t2001: F, t5232: F, t2327: F, t7610: F, t31676: F, t31680: F, t31682: F, t31684: F, t31687: F, t35924: F, t35927: F, t35928: F, t35931: F, t35935: F, t35936: F, t35938: F, t35942: F, t35947: F, t35949: F) -> F {
    let t35952 = F::new(0.17149607247227894789e-2) * t35951;
    let t35953 = t2001 * t5232;
    let t35955 = t7610 * t2327;
    let t35957 = F::new(13.0) / F::new(288.0) * t35924 + t35927 - F::new(0.34299214494455789578e-2) * t35928 - t35931 - t35935 - F::new(0.19865625e0) * t35936 - F::new(0.1324375e0) * t35938 - F::new(0.34299214494455789578e-2) * t31676 + F::new(0.85748036236139473944e-3) * t31680 - F::new(0.17149607247227894789e-2) * t35942 + F::new(0.55907719625962937008e-2) * t31682 - F::new(0.62896184579208304136e-3) * t31684 - F::new(0.14291339372689912324e-3) * t31687 + F::new(0.85748036236139473944e-3) * t35947 - F::new(0.85748036236139473944e-3) * t35949 - t35952 - F::new(0.85748036236139473944e-3) * t35953 + F::new(0.10718504529517434243e-3) * t35955;
    t35957
}
