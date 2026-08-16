//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 565/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk565<F: Float>(t1852: F, t4551: F, t83: F, t447: F, t925: F, t986: F, t110: F, t4462: F, t1866: F, t4454: F, t1871: F, t4436: F) -> (F, F, F, F, F, F) {
    let t4552 = t1852 * t4551;
    let t4553 = t83 * t4552;
    let t4557 = t447 * t986 * t925;
    let t4561 = t447 * t110 * t4462;
    let t4565 = t1866 * t110 * t4454;
    let t4569 = t1871 * t110 * t4436;
    (t4552, t4553, t4557, t4561, t4565, t4569)
}
