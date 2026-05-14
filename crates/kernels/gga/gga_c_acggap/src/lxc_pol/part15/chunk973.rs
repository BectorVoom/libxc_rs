//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 973/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk973<F: Float>(t6265: F, t8511: F, t2030: F, t5906: F, t7815: F, t6289: F, t7440: F, t9666: F, t1988: F, t9554: F, t1782: F, t355: F, t1980: F, t5011: F, t7458: F, t2001: F, t5821: F) -> (F, F, F, F, F, F, F, F) {
    let t39054 = t8511 * t6265;
    let t39057 = t2030 * t7815 * t5906;
    let t39060 = t2030 * t7815 * t6289;
    let t39062 = t7440 * t9666;
    let t39064 = t1988 * t9554;
    let t39066 = t355 * t1782;
    let t39069 = t1980 * t7458 * t5011 * t39066;
    let t39071 = t2001 * t5821;
    (t39054, t39057, t39060, t39062, t39064, t39066, t39069, t39071)
}
