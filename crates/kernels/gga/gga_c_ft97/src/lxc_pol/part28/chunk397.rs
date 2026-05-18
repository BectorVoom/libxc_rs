//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 397/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk397<F: Float>(t1359: F, t590: F, t586: F, t28: F, t5890: F, t1369: F, t1370: F, t376: F, t1368: F, t92: F) -> (F, F, F, F, F) {
    let t5891 = t1359 * t590;
    let t5892 = t586 * t5891;
    let t5894 = t5890 * t28 * t5892;
    let t5897 = t1369 * t376 * t1370;
    let t5898 = t5897 / F::new(18.0);
    let t5899 = t1368 * t92;
    (t5892, t5894, t5897, t5898, t5899)
}
