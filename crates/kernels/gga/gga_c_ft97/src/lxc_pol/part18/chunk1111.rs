//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1111/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1111<F: Float>(t1286: F, t1637: F, t5509: F, t497: F, t5617: F, t1642: F, t5507: F, t22910: F, t22914: F, t22878: F, t5495: F, t1307: F, t1920: F, t22919: F, t22924: F, t22928: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93864 = t1286 * t1637 * t5509;
    let t93866 = t5617 * t497;
    let t93871 = t1642 * t5507;
    let t93882 = t22914 * t22910;
    let t93888 = t5495 * t22878;
    let t93910 = t1307 * t1920;
    let t93915 = t22914 * t22919;
    let t93923 = t22914 * t22924;
    let t93925 = t22914 * t22928;
    (t93864, t93866, t93871, t93882, t93888, t93910, t93915, t93923, t93925)
}
