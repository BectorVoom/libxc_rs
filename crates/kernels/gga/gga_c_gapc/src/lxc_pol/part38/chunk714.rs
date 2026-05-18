//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 714/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk714<F: Float>(t433: F, t463: F, t1567: F, t2890: F, t1001: F, t8422: F, t1901: F) -> (F, F, F) {
    let t8442 = t463 * t433;
    let t8443 = t2890 * t1567;
    let t8444 = t8442 * t8443;
    let t8446 = t8422 * t1001;
    let t8448 = F::new(1.0) / t1901;
    (t8444, t8446, t8448)
}
