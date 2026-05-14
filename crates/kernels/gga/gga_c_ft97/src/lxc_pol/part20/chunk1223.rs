//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1223/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1223<F: Float>(t10409: F, t113051: F, t446: F, t113056: F, t43468: F, t14116: F, t25037: F, t3281: F, t1476: F, t15056: F, t193: F, t6308: F, t852: F, t4226: F, t6260: F, t113120: F, t113124: F, t113128: F, t113131: F, t113135: F, t113139: F, t113144: F) -> (F, F, F, F, F, F, F) {
    let t113147 = t446 * t10409 * t113051;
    let t113150 = t446 * t43468 * t113056;
    let t113152 = t25037 * t14116;
    let t113154 = t3281 * t10409 * t113152;
    let t113159 = t6308 * t193 * t852 * t1476 * t15056;
    let t113164 = t6308 * t193 * t852 * t6260 * t4226;
    let t113166 = t113120 / 9.0 - 12.0 * t113124 - 4.0 / 3.0 * t113128 - 4.0 / 3.0 * t113131 - 6.0 * t113135 + t113139 / 3.0 + 2.0 / 3.0 * t113144 + 2.0 / 9.0 * t113147 + 10.0 / 27.0 * t113150 - 8.0 / 9.0 * t113154 + t113159 / 4.0 + t113164 / 2.0;
    (t113147, t113150, t113152, t113154, t113159, t113164, t113166)
}
