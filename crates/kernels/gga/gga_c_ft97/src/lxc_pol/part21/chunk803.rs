//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 803/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk803<F: Float>(t1355: F, t22849: F, t22856: F, t1369: F, t376: F, t5909: F, t1359: F, t1570: F) -> (F, F, F, F) {
    let t23874 = t1355 * t22849;
    let t23877 = 0.11113000182098765433e-1 * t1355 * t22856;
    let t23890 = t1369 * t376 * t5909;
    let t23892 = t1359 * t1570;
    (t23874, t23877, t23890, t23892)
}
