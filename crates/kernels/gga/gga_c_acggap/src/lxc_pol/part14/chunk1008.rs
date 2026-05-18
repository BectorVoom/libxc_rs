//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1008/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1008<F: Float>(t35594: F, t7433: F, t8481: F, t34161: F, t8465: F, t1992: F, t7585: F, t7842: F, t8402: F, t8787: F, t31362: F, t8956: F) -> (F, F, F, F, F, F) {
    let t35595 = F::new(0.42874018118069736972e-3) * t35594;
    let t35596 = t7433 * t8481;
    let t35597 = F::new(0.12862205435420921092e-2) * t35596;
    let t35601 = t34161 * t8465;
    let t35602 = F::new(0.56606566121287473722e-1) * t35601;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    let t35609 = F::new(0.20965394859736101378e-3) * t35608;
    let t35610 = t7433 * t8787;
    let t35611 = F::new(0.56606566121287473722e-2) * t35610;
    let t35616 = t31362 * t8956;
    (t35595, t35597, t35602, t35609, t35611, t35616)
}
