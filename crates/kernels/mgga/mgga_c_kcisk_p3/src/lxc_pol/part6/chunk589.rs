//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 589/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk589<F: Float>(t467: F, t8231: F, t488: F, t2263: F, t6388: F, t2259: F, t2267: F, t470: F, t8072: F, t487: F, t4209: F, t4204: F, t8077: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t8232 = t8231 * t467;
    let t8233 = t8232 * sigma0;
    let t8234 = t8233 * t488;
    let t8236 = t6388 * t2263;
    let t8238 = t2259 * t2267;
    let t8240 = t470 * t8072;
    let t8241 = t487 * t8240;
    let t8242 = t4209 * t8241;
    let t8244 = t4204 * t8077;
    (t8233, t8234, t8236, t8238, t8240, t8241, t8242, t8244)
}
