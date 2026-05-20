//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3396/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396<F: Float>(t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F, t63469: F, t63471: F) -> F {
    let t63764 = F::cast_from(0.10735111111111111112e1_f64) * t52035 - F::cast_from(0.35783703703703703705e0_f64) * t52037 - F::cast_from(0.80513333333333333336e0_f64) * t52039 - F::cast_from(0.40256666666666666668e0_f64) * t52041 - F::cast_from(0.80513333333333333335e0_f64) * t52045 + F::cast_from(0.26837777777777777778e0_f64) * t52047 + F::cast_from(0.13418888888888888889e0_f64) * t52049 + F::cast_from(0.22364814814814814815e0_f64) * t52051 + F::new(0.11038e0) * t52065 - F::cast_from(0.14717333333333333333e0_f64) * t63393 + F::new(0.16504875e0) * t63396 - F::new(0.72462e1) * t63399 + F::new(0.16504875e0) * t63469 + F::new(0.19419375e1) * t63471;
    t63764
}
