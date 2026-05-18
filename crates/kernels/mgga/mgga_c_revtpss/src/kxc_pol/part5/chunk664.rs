//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 664/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk664<F: Float>(t4210: F, t606: F, t4186: F, t60: F, t1474: F, t1480: F, t2290: F, t4202: F, t4205: F, t44: F, t56: F, t614: F, t620: F) -> (F, F, F) {
    let t4211 = t4210 * t606;
    let t4214 = t60 * t4186;
    let t4217 = -F::new(20.0) / F::new(9.0) * t614 * t1474 + F::new(5.0) / F::new(18.0) * t44 * t4202 + F::new(5.0) / F::new(6.0) * t44 * t4205 + F::new(20.0) / F::new(9.0) * t1480 * t620 + F::new(5.0) / F::new(18.0) * t56 * t4211 - F::new(5.0) / F::new(6.0) * t56 * t4214 - t2290;
    (t4211, t4214, t4217)
}
