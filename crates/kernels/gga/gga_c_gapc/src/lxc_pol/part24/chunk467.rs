//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 467/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk467<F: Float>(t1007: F, t518: F, t2880: F, t568: F, t120: F, t1539: F, t5: F) -> (F, F, F, F) {
    let t2897 = t518 * t1007;
    let t2899 = t2880 * t568;
    let t2900 = t120 * t2899;
    let t2902 = t5 * t1539;
    (t2897, t2899, t2900, t2902)
}
