//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 923/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk923<F: Float>(t137112: F, t1564: F, t5674: F, t925: F, t1871: F, t22952: F, t25888: F, t32115: F, t25893: F, t25894: F, t452: F, t25990: F, t5675: F, t8411: F, t23054: F, t34394: F) -> (F, F, F, F, F) {
    let t145607 = t5674 * t1564 * t137112 * t925;
    let t145611 = t22952 * t1871 * t32115 * t25888;
    let t145615 = t25893 * t452 * t32115 * t25894;
    let t145619 = t5674 * t8411 * t5675 * t25990;
    let t145621 = t23054 * t34394;
    (t145607, t145611, t145615, t145619, t145621)
}
