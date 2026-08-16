//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 556/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk556<F: Float>(t1317: F, t376: F, t5680: F, t1307: F, t1557: F, t1882: F, t5693: F, t358: F, t5617: F, t8345: F, t91: F, t5665: F, t5667: F) -> (F, F, F, F, F, F) {
    let t22980 = t1317 * t376 * t5680;
    let t22986 = t1307 * t1557;
    let t22991 = t1882 * t5693;
    let t22993 = t5617 * t358;
    let t23008 = t91 * t8345;
    let t23016 = t5665 * t376 * t5667;
    (t22980, t22986, t22991, t22993, t23008, t23016)
}
