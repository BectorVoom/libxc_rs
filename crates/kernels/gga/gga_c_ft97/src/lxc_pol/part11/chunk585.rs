//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 585/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk585<F: Float>(t110: F, t1871: F, t8544: F, t1820: F, t432: F, t452: F, t488: F, t1755: F, t492: F, t1786: F, t1876: F, t379: F, t1853: F, t1852: F, t1859: F, t1882: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8546 = t1871 * t110 * t8544;
    let t8549 = t1820 * t432;
    let t8551 = t452 * t488 * t8549;
    let t8553 = t1755 * t492;
    let t8555 = t452 * t488 * t8553;
    let t8557 = t1786 * t488;
    let t8558 = t1876 * t379;
    let t8559 = t8557 * t8558;
    let t8562 = t1853 * t432;
    let t8564 = t452 * t1852 * t8562;
    let t8567 = t1882 * t1859;
    (t8546, t8549, t8551, t8553, t8555, t8557, t8558, t8559, t8562, t8564, t8567)
}
