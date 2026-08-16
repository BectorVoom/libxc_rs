//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 606/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk606<F: Float>(t1487: F, t322: F, t368: F, t398: F, t384: F, t1140: F, t1507: F, t145: F, t301: F, t336: F, t3570: F, t500: F) -> (F, F, F, F, F, F, F) {
    let t4623 = t1487 * t322;
    let t4625 = t398 * t368 * t4623;
    let t4627 = F::cast_from(0.85748036236139473944e-3_f64) * t384 * t4625;
    let t4629 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t1140 * t1507;
    let t4630 = t1487 * t145;
    let t4632 = t336 * t4630 * t301;
    let t4635 = t3570 * t500;
    (t4623, t4625, t4627, t4629, t4630, t4632, t4635)
}
