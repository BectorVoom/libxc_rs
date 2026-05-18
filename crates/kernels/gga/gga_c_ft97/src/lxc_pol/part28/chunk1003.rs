//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1003/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1003<F: Float>(t1526: F, t6512: F, t7705: F, t342: F, t34607: F, t630: F, t1774: F, t6520: F, t7151: F, t32029: F, t34592: F, t1286: F, t1546: F, t34595: F) -> (F, F, F, F, F) {
    let t144505 = t1526 * t7705 * t6512;
    let t144511 = t342 * t630 * t34607;
    let t144520 = t7151 * t1774 * t6520;
    let t144524 = t34592 * t32029;
    let t144538 = t1286 * t1546 * t34595;
    (t144505, t144511, t144520, t144524, t144538)
}
