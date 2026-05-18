//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 888/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk888<F: Float>(t13140: F, t35067: F, t1901: F, t33008: F, t33016: F, t33066: F, t35035: F, t35039: F, t35043: F, t35047: F, t35052: F, t35056: F, t35060: F, t35064: F, t446: F) -> (F, F) {
    let t35068 = t13140 * t35067;
    let t35071 = t33008 + F::new(4.0) / F::new(3.0) * t446 * t35035 + F::new(4.0) / F::new(3.0) * t446 * t35039 + F::new(2.0) / F::new(3.0) * t446 * t35043 + t33016 - t446 * t35047 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t446 * t35052 + F::new(4.0) / F::new(3.0) * t446 * t35056 + F::new(2.0) / F::new(3.0) * t446 * t35060 - t33066 + t1901 * t35064 / F::new(9.0) - F::new(4.0) / F::new(3.0) * t1901 * t35068;
    (t35068, t35071)
}
