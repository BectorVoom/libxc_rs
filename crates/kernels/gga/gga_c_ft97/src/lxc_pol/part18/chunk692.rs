//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 692/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk692<F: Float>(t3219: F, t379: F, t11854: F, t1876: F, t925: F, t8557: F, t110: F, t8216: F) -> (F, F, F, F, F) {
    let t11855 = t3219 * t379;
    let t11856 = t11854 * t11855;
    let t11859 = t925 * t1876;
    let t11860 = t8557 * t11859;
    let t11863 = t8216 * t110;
    (t11855, t11856, t11859, t11860, t11863)
}
