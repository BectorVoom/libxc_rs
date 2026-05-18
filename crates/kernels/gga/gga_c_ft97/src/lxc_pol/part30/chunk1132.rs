//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1132/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1132<F: Float>(t292: F, t153020: F, t153066: F, t153129: F, t153183: F, t153229: F, t153267: F, t153325: F, t153368: F, t1486: F, t193: F, t852: F, t10683: F, t35833: F, t446: F, t824: F) -> (F, F, F) {
    let t293 = F::new(0.1e-59) < t292;
    let t153372 = piecewise3::<f64>(t293, t153020 + t153066 + t153129 + t153183 + t153229 + t153267 + t153325 + t153368, F::new(0.0));
    let t153375 = t1486 * t193 * t852 * t153372;
    let t153379 = t446 * t10683 * t35833 * t824;
    (t153372, t153375, t153379)
}
