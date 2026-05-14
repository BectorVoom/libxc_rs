//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 264/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk264<F: Float>(t1866: F, t2984: F, t446: F, t432: F, t925: F, t1564: F, t1570: F, t920: F, t363: F) -> (F, F, F, F) {
    let t2985 = t1866 * t2984;
    let t2986 = t446 * t2985;
    let t2988 = t925 * t432;
    let t2989 = t1564 * t2988;
    let t2990 = t446 * t2989;
    let t2992 = t1570 * t920;
    let t2993 = t2992 * t363;
    (t2986, t2988, t2990, t2993)
}
