//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 779/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk779<F: Float>(t7464: F, t7466: F, t7468: F, t7479: F, t7481: F, t7484: F, t7488: F, t7496: F, t7500: F, t7516: F, t7520: F, t8740: F, t8742: F, t8744: F, t8748: F) -> F {
    let t8750 = F::new(0.18868855373762491241e-2) * t7464 - F::new(0.28303283060643736861e-2) * t7466 + F::new(0.7862023072401038017e-3) * t7468 + F::new(0.52413487149340253445e-3) * t7479 - F::new(0.31448092289604152068e-3) * t7481 + F::new(0.22921875e-1) * t7484 + F::new(0.1528125e-1) * t7488 - F::new(0.7862023072401038017e-3) * t7496 + F::new(0.31448092289604152068e-3) * t7500 + F::new(0.31448092289604152068e-3) * t8740 + F::new(0.22921875e-1) * t8742 + F::new(0.1528125e-1) * t8744 + t7516 - t7520 - F::new(0.38203125e-2) * t8748;
    t8750
}
