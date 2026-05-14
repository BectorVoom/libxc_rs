//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 723/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk723<F: Float>(t590: F, t7312: F, t7369: F, t32888: F, t7239: F, t32063: F, t7366: F, t7370: F, t5889: F, t631: F) -> (F, F, F, F, F, F) {
    let t32889 = t7312 * t590;
    let t32890 = t7369 * t32889;
    let t32892 = t32888 * t7239 * t32890;
    let t32895 = t7366 * t32063 * t7370;
    let t32896 = 2.0 / 3.0 * t32895;
    let t32897 = t5889 * t631;
    (t32889, t32890, t32892, t32895, t32896, t32897)
}
