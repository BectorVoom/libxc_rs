//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 651/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk651<F: Float>(t11906: F, t5718: F, t23249: F, t3214: F, t11490: F, t23: F, t82: F, t100: F, t1332: F, t8417: F, t3219: F, t3266: F, t5717: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26364 = t11906 * t5718;
    let t26367 = t23249 * t3214;
    let t26368 = t11490 * t26367;
    let t26371 = t23 * t82;
    let t26372 = t26371 * t100;
    let t26373 = t8417 * t1332;
    let t26374 = t26373 * t3219;
    let t26375 = t26372 * t26374;
    let t26378 = t5717 * t3266;
    (t26364, t26367, t26368, t26371, t26372, t26373, t26374, t26375, t26378)
}
