//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 649/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk649<F: Float>(t24789: F, t3876: F, t1901: F, t193: F, t24841: F, t24843: F, t28417: F, t28422: F, t28426: F, t28430: F, t28434: F, t28438: F, t28441: F, t28445: F, t28448: F, t28451: F, t28453: F, t446: F, t89: F) -> F {
    let t28455 = t24789 * t3876;
    let t28458 = t24841 / F::new(9.0) + t24843 / F::new(9.0) + t89 * t193 * t28417 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t28422 + t446 * t28426 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t28430 + F::new(2.0) / F::new(3.0) * t446 * t28434 + F::new(2.0) / F::new(3.0) * t446 * t28438 + F::new(2.0) / F::new(3.0) * t446 * t28441 + t446 * t28445 / F::new(3.0) - t446 * t28448 / F::new(3.0) + t28451 / F::new(9.0) + t28453 / F::new(9.0) + t1901 * t28455 / F::new(9.0);
    t28458
}
