//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 891/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk891<F: Float>(t1017: F, t7407: F, t574: F, t605: F, t1901: F, t33146: F, t35073: F, t35076: F, t35080: F, t35084: F, t35087: F, t35091: F, t35095: F, t35099: F, t35103: F, t35107: F, t446: F) -> (F, F, F) {
    let t35110 = t7407 * t1017;
    let t35112 = t574 * t605 * t35110;
    let t35115 = F::new(2.0) / F::new(3.0) * t446 * t35073 + F::new(2.0) / F::new(9.0) * t1901 * t35076 - F::new(4.0) / F::new(3.0) * t1901 * t35080 - F::new(2.0) / F::new(9.0) * t1901 * t35084 + F::new(2.0) / F::new(9.0) * t1901 * t35087 + t1901 * t35091 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t35095 + t33146 + F::new(2.0) / F::new(3.0) * t446 * t35099 + F::new(4.0) / F::new(3.0) * t446 * t35103 - F::new(2.0) * t446 * t35107 + t446 * t35112 / F::new(3.0);
    (t35110, t35112, t35115)
}
