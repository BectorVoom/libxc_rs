//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1348/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1348<F: Float>(t12561: F, t1359: F, t1369: F, t2112: F, t28: F, t23649: F, t27138: F, t1643: F, t27165: F, t5899: F, t9049: F, t105919: F, t105926: F, t105930: F, t105935: F, t95369: F, t95378: F, t96137: F, t96141: F, t96146: F) -> (F, F, F, F, F) {
    let t105937 = t1359 * t12561;
    let t105940 = t1369 * t28 * t2112 * t105937;
    let t105941 = t23649 * t27138;
    let t105942 = t105941 / 9.0;
    let t105945 = t5899 * t9049 * t27165 * t1643;
    let t105947 = 3.0 / 2.0 * t105919 - t96137 + t105926 / 4.0 + t105930 / 3.0 + t95369 - t96141 + t105935 / 4.0 + t105940 + t95378 - t105942 - t96146 + t105945 / 9.0;
    (t105937, t105940, t105941, t105945, t105947)
}
