//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1096/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1096<F: Float>(t6119: F, t9895: F, t1424: F, t9577: F, t6137: F, t8232: F, t1434: F, t2399: F, t6128: F, t9570: F, t1433: F, t1771: F, t6121: F, t1435: F, t9555: F, t6140: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96935 = t9895 * t6119;
    let t96945 = t1424 * t9577;
    let t96953 = t8232 * t6137;
    let t96958 = t1434 * t2399 * t6128;
    let t96970 = t1424 * t9570;
    let t96982 = t1433 * t1771;
    let t96983 = t96982 * t6121;
    let t97029 = t1434 * t9555 * t1435;
    let t97030 = 14.0 / 27.0 * t97029;
    let t97061 = t89 * t2399 * t6140;
    (t96935, t96945, t96953, t96958, t96970, t96982, t96983, t97029, t97030, t97061)
}
