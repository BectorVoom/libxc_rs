//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2139/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2139<F: Float>(t33: F, t22778: F, t22783: F, t516: F, t5557: F, t6416: F, t9350: F, t162: F, t22777: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t22787 = piecewise3::<F>(t34, F::new(0.0), -F::new(8.0) / F::new(27.0) * t9350 * t22778 + F::new(4.0) / F::new(3.0) * t5557 * t6416 + F::new(4.0) / F::new(3.0) * t516 * t22783);
    let t22789 = (t22777 + t22787) * t162;
    t22789
}
