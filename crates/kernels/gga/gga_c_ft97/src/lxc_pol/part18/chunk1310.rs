//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1310/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1310<F: Float>(t13135: F, t1384: F, t2179: F, t26526: F, t9276: F, t586: F, t7954: F, t11437: F, t95312: F, t5899: F, t11982: F, t23892: F, t23671: F, t95332: F, t27072: F, t23909: F) -> (F, F, F, F, F, F, F, F, F) {
    let t105336 = t2179 * t1384 * t13135;
    let t105338 = t9276 * t26526;
    let t105340 = t7954 * t586;
    let t105341 = t95312 * t11437;
    let t105343 = t5899 * t105340 * t105341;
    let t105345 = t23892 * t11982;
    let t105347 = t5899 * t23671 * t105345;
    let t105349 = t95332 * t11437;
    let t105351 = t5899 * t27072 * t105349;
    let t105353 = t23909 * t11982;
    (t105336, t105338, t105341, t105343, t105345, t105347, t105349, t105351, t105353)
}
