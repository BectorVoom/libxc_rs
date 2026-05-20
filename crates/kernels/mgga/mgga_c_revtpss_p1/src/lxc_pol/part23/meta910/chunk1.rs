//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2924/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924<F: Float>(t77778: F, t77797: F, t923: F, t52035: F, t52037: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t77539: F, t77543: F, t77547: F) -> (F, F, F) {
    let t77798 = t77778 + t77797;
    let t77799 = t923 * t77798;
    let t77801 = -F::new(0.543465e1) * t77539 + F::new(0.181155e1) * t77543 + F::new(0.181155e1) * t77547 - F::new(0.12077e1) * t63338 + F::cast_from(0.40256666666666666666e0_f64) * t63340 + F::cast_from(0.33547222222222222222e0_f64) * t63342 + F::new(0.181155e1) * t63361 - F::new(0.12077e1) * t63371 + F::cast_from(0.80513333333333333336e0_f64) * t52035 - F::cast_from(0.26837777777777777779e0_f64) * t52037 + F::new(0.16504875e0) * t77799;
    (t77798, t77799, t77801)
}
