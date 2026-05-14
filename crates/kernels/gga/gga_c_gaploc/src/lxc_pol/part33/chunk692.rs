//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 692/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk692<F: Float>(t1407: F, t2467: F, t1: F, t6514: F, t1415: F, t1391: F, t2466: F, t587: F, t1323: F, t487: F, t2365: F, t4361: F, t600: F, t6393: F, t568: F, t1508: F, t894: F) -> (F, F, F, F, F, F, F) {
    let t6849 = t1407 * t2467;
    let t6851 = t6514 * t1;
    let t6852 = t1415 * t6851;
    let t6855 = t1391 * t2466;
    let t6856 = t587 * t6855;
    let t6858 = t487 * t1323;
    let t6859 = t2365 * t6858;
    let t6860 = t4361 * t6859;
    let t6862 = t600 * t6393;
    let t6863 = t568 * t6862;
    let t6866 = t1508 * t894;
    (t6849, t6851, t6852, t6856, t6860, t6863, t6866)
}
