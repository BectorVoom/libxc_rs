//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 680/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk680<F: Float>(t2016: F, t2056: F, t1156: F, t570: F, t1981: F, t576: F, t579: F, t922: F, t336: F, t2020: F, t374: F, t1145: F, t2041: F) -> (F, F, F, F, F, F, F, F) {
    let t7396 = t2016 * t2056;
    let t7397 = F::cast_from(0.28015625e-1_f64) * t7396;
    let t7398 = t570 * t1156;
    let t7400 = t576 * t1981;
    let t7401 = t579 * t922;
    let t7402 = t336 * t7401;
    let t7403 = t7400 * t7402;
    let t7405 = t2020 * t374;
    let t7406 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7405;
    let t7407 = t2041 * t1145;
    (t7396, t7397, t7398, t7402, t7403, t7405, t7406, t7407)
}
