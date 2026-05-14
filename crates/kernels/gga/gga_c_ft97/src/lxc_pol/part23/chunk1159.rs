//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1159/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1159<F: Float>(t109168: F, t1472: F, t109015: F, t28558: F, t1471: F, t4061: F, t2035: F, t6789: F) -> (F, F, F, F) {
    let t112205 = t1472 * t109168;
    let t112219 = 0.26853068634149852184e-1 * t28558 * t109015;
    let t112220 = t4061 * t1471;
    let t112223 = t2035 * t6789;
    (t112205, t112219, t112220, t112223)
}
