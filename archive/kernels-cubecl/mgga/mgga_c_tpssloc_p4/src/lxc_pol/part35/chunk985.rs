//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 985/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk985<F: Float>(t4295: F, t5617: F, t16891: F, t2645: F, t5591: F, t232: F, t5544: F, t4181: F, t1510: F, t4180: F, t20756: F, t820: F, t9607: F) -> (F, F, F, F, F) {
    let t20876 = t4295 * t5617;
    let t20882 = t2645 * t16891 * t5591;
    let t20885 = t232 * t5544;
    let t20887 = t2645 * t4181 * t20885;
    let t20891 = t4180 * t16891 * t1510;
    let t20896 = t9607 * t820 * t20756;
    (t20876, t20882, t20887, t20891, t20896)
}
