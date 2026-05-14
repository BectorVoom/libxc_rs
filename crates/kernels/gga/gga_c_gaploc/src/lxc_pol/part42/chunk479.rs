//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 479/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk479<F: Float>(t7068: F, t883: F, t1967: F, t7810: F, t7403: F, t959: F, t7340: F, t3281: F, t5676: F, t2530: F, t2610: F, t2365: F, t2033: F, t1645: F, t2586: F, t3307: F, t9420: F) -> (F, F, F, F, F, F, F, F) {
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9935 = 0.29792074959875355558e-1 * t7403 * t959;
    let t9937 = 0.29792074959875355558e-1 * t7340 * t959;
    let t9942 = 0.29792074959875355558e-1 * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = 0.29792074959875355558e-1 * t2033 * t9944;
    let t9972 = t1645 * t2586;
    let t9981 = t9420 * t3307;
    (t9889, t9891, t9935, t9937, t9942, t9946, t9972, t9981)
}
