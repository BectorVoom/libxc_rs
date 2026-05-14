//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1251/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1251<F: Float>(t108187: F, t1154: F, t2360: F, t3886: F, t96934: F, t110125: F, t110128: F, t110129: F, t124020: F, t124026: F, t124031: F, t124036: F, t124040: F, t96953: F, t96958: F, t96983: F) -> (F, F) {
    let t124045 = t96934 * t108187 * t1154 * t2360 * t3886;
    let t124047 = -4.0 / 27.0 * t124020 + 4.0 / 81.0 * t96953 - 2.0 / 27.0 * t96958 - t124026 / 18.0 + 2.0 / 81.0 * t96983 + 2.0 / 9.0 * t124031 - t110125 + t110128 + t110129 - t124036 / 6.0 + t124040 / 9.0 + t124045 / 9.0;
    (t124045, t124047)
}
