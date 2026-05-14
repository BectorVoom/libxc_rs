//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 729/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk729<F: Float>(t1969: F, t32946: F, t379: F, t5899: F, t558: F, t7339: F, t2112: F, t1369: F, t28: F, t32869: F, t586: F, t375: F, t7382: F, t89: F, t358: F, t7312: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32948 = t1969 * t32946 * t379;
    let t32949 = t5899 * t32948;
    let t32951 = t7339 * t558;
    let t32952 = t2112 * t32951;
    let t32954 = t1369 * t28 * t32952;
    let t32955 = t586 * t32869;
    let t32957 = t1369 * t28 * t32955;
    let t32960 = t89 * t375 * t7382;
    let t32961 = 2.0 / 3.0 * t32960;
    let t32962 = t7312 * t358;
    (t32948, t32949, t32951, t32952, t32954, t32955, t32957, t32960, t32961, t32962)
}
