//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 732/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk732<F: Float>(t13073: F, t13079: F, t13113: F, t13114: F, t13115: F, t13116: F, t13117: F, t13120: F, t13886: F, t13890: F, t13893: F, t13895: F) -> F {
    let t14511 = -t13886 - t13890 - F::new(0.29792074959875355558e-1) * t13893 + F::new(0.29792074959875355558e-1) * t13895 - F::new(0.89376224879626066674e-1) * t13073 + t13079 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120;
    t14511
}
