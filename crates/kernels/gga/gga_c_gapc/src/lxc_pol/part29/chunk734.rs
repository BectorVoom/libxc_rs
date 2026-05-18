//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 734/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk734<F: Float>(t1743: F, t8624: F, t5722: F, t458: F, t4925: F, t3104: F, t568: F, t3108: F, t1027: F, t1842: F, t1036: F, t1716: F) -> (F, F, F, F, F) {
    let t8625 = t1743 * t8624;
    let t8626 = t8625 * t5722;
    let t8628 = t4925 * t458;
    let t8629 = t3104 * t8628;
    let t8631 = t4925 * t568;
    let t8632 = t3108 * t8631;
    let t8634 = t1027 * t1842;
    let t8636 = t1036 * t1716;
    (t8626, t8629, t8632, t8634, t8636)
}
