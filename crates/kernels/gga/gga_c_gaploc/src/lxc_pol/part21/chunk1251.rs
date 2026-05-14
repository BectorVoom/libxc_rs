//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1251/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1251<F: Float>(t28129: F, t28150: F, t32907: F, t32910: F, t32911: F, t32923: F, t32925: F, t32927: F, t32928: F, t32931: F, t32935: F, t32936: F, t32938: F, t32940: F, t32942: F, t12161: F, t835: F) -> (F, F) {
    let t38958 = -t32907 + t32910 - t32911 + t32923 + t32925 + t32927 + t28129 + t32928 + t32931 - 0.76685851907841499354e0 * t28150 + t32935 + t32936 + t32938 + t32940 + t32942;
    let t38961 = t835 * t12161;
    (t38958, t38961)
}
