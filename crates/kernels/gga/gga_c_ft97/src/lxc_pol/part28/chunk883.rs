//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 883/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk883<F: Float>(t1349: F, t1526: F, t1527: F, t2: F, t32663: F, t32675: F, t342: F, t343: F, t34985: F, t34989: F, t34994: F, t35000: F, t6673: F, t6678: F, t7298: F, t7299: F) -> F {
    let t35005 = (-t34985 * t7299 / F::new(6.0) + t32663 + t1349 * t34989 / F::new(18.0) + t1349 * t6678 / F::new(3.0) - t7298 * t34994 / F::new(6.0) - t32675 - t1526 * t1527 * t6673 / F::new(12.0) - t342 * t343 * t35000 / F::new(4.0)) * t2;
    t35005
}
