//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 571/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk571<F: Float>(t50: F, t34: F, t829: F, t1289: F, t1292: F, t296: F, t39: F, t4015: F, t4084: F, t821: F, t830: F, t833: F, t4083: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t4087 = t829 * t34;
    let t4097 = piecewise3::<f64>(t51, F::new(0.0), F::new(8.0) / F::new(27.0) * t4084 * t830 + F::new(8.0) / F::new(9.0) * t4087 * t4015 - F::new(2.0) / F::new(9.0) * t1289 * t833 - F::new(4.0) / F::new(3.0) * t296 * t821 + F::new(4.0) * t1292 * t39);
    let t4099 = t4083 / F::new(2.0) + t4097 / F::new(2.0);
    t4099
}
