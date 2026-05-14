//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 727/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk727<F: Float>(t32924: F, t379: F, t9073: F, t5899: F, t32907: F, t9236: F, t1369: F, t28: F, t2112: F, t32912: F, t590: F, t7339: F, t586: F, t5890: F, t32892: F, t32896: F, t32902: F, t32910: F, t32915: F, t32919: F, t32923: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32926 = t9073 * t32924 * t379;
    let t32927 = t5899 * t32926;
    let t32929 = t9236 * t32907;
    let t32931 = t1369 * t28 * t32929;
    let t32933 = t2112 * t32912;
    let t32935 = t1369 * t28 * t32933;
    let t32937 = t7339 * t590;
    let t32938 = t586 * t32937;
    let t32940 = t5890 * t28 * t32938;
    let t32942 = 3.0 / 2.0 * t32892 + t32896 + 2.0 / 3.0 * t32902 + 4.0 * t32910 - 2.0 * t32915 - t32919 / 2.0 - t32923 - t32927 / 3.0 - 3.0 * t32931 + 2.0 * t32935 + t32940 / 4.0;
    (t32926, t32927, t32929, t32931, t32933, t32935, t32938, t32940, t32942)
}
