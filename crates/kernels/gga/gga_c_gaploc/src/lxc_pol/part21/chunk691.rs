//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 691/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk691<F: Float>(t2318: F, t6498: F, t2322: F, t484: F, t2310: F, t423: F, t481: F, t4260: F, t486: F) -> (F, F, F, F) {
    let t6499 = t6498 * t2318;
    let t6501 = t484 * t2322;
    let t6504 = t481 * t2310 * t423;
    let t6505 = t6504 * t2318;
    let t6507 = t4260 * t486;
    (t6499, t6501, t6505, t6507)
}
