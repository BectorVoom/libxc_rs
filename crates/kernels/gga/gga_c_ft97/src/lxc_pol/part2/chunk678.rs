//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 678/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk678<F: Float>(t11076: F, t11416: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t11783: F, t11787: F, t11791: F, t8260: F, t11928: F, t11936: F, t11948: F, t488: F, t83: F) -> (F, F) {
    let t11949 = 4.0 / 9.0 * t11076;
    let t11957 = 4.0 / 3.0 * t11416;
    let t11958 = -t11949 - t8260 - t11783 / 4.0 + 3.0 / 8.0 * t11787 - t11791 / 2.0 - t11395 - 4.0 / 3.0 * t11399 + 22.0 / 9.0 * t11404 + 2.0 * t11408 + 4.0 * t11413 - t11957;
    let t11960 = t11928 + t11936 + t11948 + t11958;
    let t11961 = t488 * t11960;
    let t11962 = t83 * t11961;
    (t11961, t11962)
}
