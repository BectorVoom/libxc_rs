//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 977/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk977<F: Float>(t1307: F, t1851: F, t3219: F, t11490: F, t100: F, t7750: F, t3266: F, t5630: F, t23339: F, t3271: F, t11810: F, t6465: F, t8372: F, t1901: F, t23148: F, t26135: F, t26139: F, t26142: F, t26147: F, t26151: F, t26156: F, t26159: F, t26163: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26166 = t1851 * t1307;
    let t26167 = t26166 * t3219;
    let t26168 = t11490 * t26167;
    let t26171 = t7750 * t100;
    let t26172 = t5630 * t3266;
    let t26173 = t26171 * t26172;
    let t26176 = t23339 * t3271;
    let t26177 = t11810 * t26176;
    let t26180 = t8372 * t6465;
    let t26183 = -2.0 / 9.0 * t1901 * t26135 + t23148 / 27.0 + t26139 / 9.0 + t446 * t26142 / 3.0 + t446 * t26147 / 3.0 + 2.0 / 3.0 * t446 * t26151 + t446 * t26156 / 3.0 + t1901 * t26159 / 9.0 - 2.0 / 3.0 * t1901 * t26163 - 2.0 / 3.0 * t1901 * t26168 - 2.0 * t1901 * t26173 - 2.0 / 3.0 * t1901 * t26177 + t1901 * t26180 / 9.0;
    (t26166, t26167, t26168, t26171, t26172, t26173, t26176, t26177, t26180, t26183)
}
