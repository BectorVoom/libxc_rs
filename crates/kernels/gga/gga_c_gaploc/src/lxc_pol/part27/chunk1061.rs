//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1061/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1061<F: Float>(t10196: F, t3818: F, t2268: F, t2304: F, t27082: F, t10253: F, t484: F, t10246: F, t6305: F, t6447: F, t8195: F, t1366: F, t2755: F, t10242: F, t1595: F, t1063: F, t21042: F, t2765: F) -> (F, F, F, F, F, F, F, F) {
    let t31924 = 0.7588001769513639893e-1 * t3818 * t10196;
    let t31928 = 0.19918504644973304719e0 * t2268 * t2304 * t27082;
    let t31929 = t484 * t10253;
    let t31930 = 0.31616674039640166222e-2 * t31929;
    let t31932 = 0.39837009289946609438e0 * t6305 * t10246;
    let t31935 = 0.39837009289946609438e0 * t2268 * t6447 * t8195;
    let t31936 = t2755 * t1366;
    let t31939 = 0.39837009289946609438e0 * t2268 * t2304 * t31936;
    let t31942 = 0.28455006635676149599e-1 * t2268 * t1595 * t10242;
    let t31945 = 0.85365019907028448797e-1 * t1063 * t2765 * t21042;
    (t31924, t31928, t31930, t31932, t31935, t31939, t31942, t31945)
}
