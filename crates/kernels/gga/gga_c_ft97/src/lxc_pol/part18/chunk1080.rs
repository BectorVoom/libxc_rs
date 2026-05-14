//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1080/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1080<F: Float>(t1882: F, t23274: F, t1326: F, t8326: F, t1786: F, t5704: F, t23324: F, t8392: F, t8216: F, t23328: F, t38953: F, t5632: F, t5637: F, t8232: F, t23280: F, t23336: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t92025 = t1882 * t23274;
    let t92035 = t8326 * t1326;
    let t92049 = t1786 * t5704;
    let t92053 = t8392 * t23324;
    let t92055 = t8216 * t1326;
    let t92059 = t8392 * t23328;
    let t92062 = t38953 * t5632;
    let t92072 = t8232 * t5637;
    let t92074 = t1882 * t23280;
    let t92086 = t8392 * t23336;
    (t92025, t92035, t92049, t92053, t92055, t92059, t92062, t92072, t92074, t92086)
}
