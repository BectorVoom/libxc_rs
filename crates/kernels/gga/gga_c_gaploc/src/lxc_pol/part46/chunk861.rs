//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 861/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk861<F: Float>(t43917: F, t13065: F, t2013: F, t43710: F, t825: F, t969: F, t41342: F, t13072: F, t32969: F, t10867: F, t41511: F, t25070: F, t7427: F, t9438: F, t1022: F, t9641: F) -> (F, F, F, F, F, F, F, F) {
    let t43918 = 0.29792074959875355558e-1 * t43917;
    let t43919 = t2013 * t13065;
    let t43922 = t825 * t969 * t43710;
    let t43924 = 0.29792074959875355558e-1 * t41342;
    let t43925 = t32969 * t13072;
    let t43926 = 0.89376224879626066675e-1 * t43925;
    let t43927 = t10867 * t41511;
    let t43928 = 0.89376224879626066675e-1 * t43927;
    let t43930 = t7427 * t9438 * t25070;
    let t43931 = 0.47928657442400937096e-1 * t43930;
    let t43932 = t9641 * t1022;
    (t43918, t43919, t43922, t43924, t43926, t43928, t43931, t43932)
}
