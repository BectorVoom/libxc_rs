//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1293/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1293<F: Float>(t1691: F, t34876: F, t420: F, t1013: F, t1701: F, t93271: F, t22652: F, t3404: F, t12374: F, t5820: F, t22632: F, t23732: F, t26639: F, t23774: F, t26643: F, t100519: F, t23701: F) -> (F, F, F, F, F, F, F) {
    let t104845 = t420 * t34876 * t1691;
    let t104851 = t1701 * t93271 * t1013;
    let t104857 = t1701 * t22652 * t3404;
    let t104860 = t12374 * t5820;
    let t104868 = 0.13335600218518518519e0 * t23732 * t22632 * t26639;
    let t104878 = 0.20003400327777777778e0 * t23774 * t22632 * t26643;
    let t104884 = 0.26853068634149852184e-1 * t23701 * t100519;
    (t104845, t104851, t104857, t104860, t104868, t104878, t104884)
}
