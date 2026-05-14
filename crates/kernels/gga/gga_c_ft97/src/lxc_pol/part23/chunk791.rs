//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 791/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk791<F: Float>(t4917: F, t9498: F, t2321: F, t4635: F, t231: F, t5053: F, t1526: F, t17685: F, t17703: F, t21103: F, t2320: F, t342: F, t343: F, t3806: F, t4915: F, t4922: F, t5059: F, t9482: F) -> (F, F, F, F) {
    let t21110 = t9498 * t4917;
    let t21114 = t2321 * t4635;
    let t21118 = t231 * t5053;
    let t21122 = t4915 + t5059 + t9482 - t17685 / 18.0 - t17703 / 6.0 - t1526 * t3806 * t21103 / 9.0 - t1526 * t2320 * t4922 / 6.0 + t1526 * t2320 * t21110 / 6.0 - t1526 * t2320 * t21114 / 12.0 - t342 * t343 * t21118 / 4.0;
    (t21110, t21114, t21118, t21122)
}
