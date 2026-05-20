//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1659/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1659<F: Float>(t12504: F, t12511: F, t435: F, t44009: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t45015: F, t45023: F, t45026: F, t45029: F, t45033: F, t45037: F, t45040: F, t45231: F, t45244: F) -> F {
    let t45251 = -t44096 - t44100 + t44103 - t44106 - t44108 + t44111 + t44114 - F::new(0.310907e-1) * (t45231 + t45244) * t435 - F::cast_from(0.19751673498613801407e-1_f64) * t44009 + t45015 - t45023 + t45026 + t45029 - t45033 - t45037 - t45040 - F::new(24.0) * t12511 * t12504;
    t45251
}
