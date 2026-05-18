//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 710/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk710<F: Float>(t1775: F, t3918: F, t3911: F, t2: F, t9952: F, t3914: F, t1148: F, t8282: F, t3932: F, t11717: F, t3922: F, t3936: F, t458: F) -> (F, F, F, F, F, F, F, F) {
    let t13306 = F::new(4.0) / F::new(9.0) * t1775 * t3918;
    let t13308 = F::new(4.0) / F::new(27.0) * t1775 * t3911;
    let t13313 = t9952 * t2;
    let t13329 = F::new(2.0) / F::new(9.0) * t1775 * t3914;
    let t13335 = t8282 * t1148;
    let t13338 = F::new(4.0) / F::new(3.0) * t1775 * t3932;
    let t13339 = t11717 * t3922;
    let t13345 = F::new(2.0) / F::new(3.0) * t458 * t3936;
    (t13306, t13308, t13313, t13329, t13335, t13338, t13339, t13345)
}
