//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 654/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk654<F: Float>(t1307: F, t3255: F, t452: F, t488: F, t3238: F, t5644: F, t1825: F, t6478: F, t5617: F, t979: F, t1882: F, t6488: F) -> (F, F, F, F, F, F, F) {
    let t26410 = t1307 * t3255;
    let t26412 = t452 * t488 * t26410;
    let t26416 = t452 * t3238 * t5644;
    let t26420 = t452 * t1825 * t6478;
    let t26423 = t5617 * t979;
    let t26425 = t452 * t488 * t26423;
    let t26428 = t1882 * t6488;
    (t26410, t26412, t26416, t26420, t26423, t26425, t26428)
}
