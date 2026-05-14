//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 966/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk966<F: Float>(t30105: F, t586: F, t1369: F, t28: F, t4431: F, t5916: F, t1969: F, t446: F, t23909: F, t4417: F, t9049: F, t27034: F, t920: F, t4458: F, t5900: F, t5899: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30191 = t586 * t30105;
    let t30193 = t1369 * t28 * t30191;
    let t30195 = t5916 * t4431;
    let t30196 = t1969 * t30195;
    let t30197 = t446 * t30196;
    let t30199 = t23909 * t4417;
    let t30200 = t9049 * t30199;
    let t30201 = t446 * t30200;
    let t30203 = t27034 * t920;
    let t30204 = t1969 * t30203;
    let t30205 = t446 * t30204;
    let t30208 = t1969 * t5900 * t4458;
    let t30209 = t5899 * t30208;
    (t30191, t30193, t30196, t30197, t30200, t30201, t30204, t30205, t30208, t30209)
}
