//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1325/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1325<F: Float>(t1900: F, t579: F, t6: F, t91: F, t40424: F, t5900: F, t12334: F, t12338: F, t95293: F, t105541: F, t105544: F, t105548: F, t105552: F, t105557: F, t105560: F, t105564: F, t105568: F, t105571: F, t105574: F) -> (F, F, F, F) {
    let t105578 = t91 * t579 * t6 * t1900;
    let t105579 = t40424 * t5900;
    let t105581 = t105578 * t105579 * t12334;
    let t105584 = t105578 * t95293 * t12338;
    let t105586 = t105541 / 6.0 + t105544 + t105548 / 8.0 - 2.0 / 3.0 * t105552 - t105557 / 2.0 + t105560 + t105564 / 4.0 - t105568 + t105571 + t105574 / 3.0 + 2.0 * t105581 + 4.0 / 3.0 * t105584;
    (t105578, t105581, t105584, t105586)
}
