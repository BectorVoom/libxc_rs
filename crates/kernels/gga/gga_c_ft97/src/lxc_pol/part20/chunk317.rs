//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 317/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk317<F: Float>(t2755: F, t2756: F, t91: F, t1771: F, t303: F, t1775: F, t849: F, t458: F, t854: F, t2344: F, t295: F) -> (F, F, F, F, F) {
    let t2758 = t91 * t2755 * t2756;
    let t2761 = 4.0 / 9.0 * t1771 * t303;
    let t2762 = t1775 * t849;
    let t2764 = t458 * t854;
    let t2766 = t2344 * t295;
    (t2758, t2761, t2762, t2764, t2766)
}
