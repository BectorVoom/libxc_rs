//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 904/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk904<F: Float>(t35580: F, t1460: F, t30159: F, t355: F, t3706: F, t7842: F, t30374: F, t8606: F, t7426: F, t7569: F, t8480: F, t7433: F, t8481: F, t34161: F, t8465: F, t1992: F, t7585: F, t8402: F) -> (F, F, F, F, F, F, F) {
    let t35581 = 0.12579236915841660827e-2 * t35580;
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    let t35586 = 0.25158473831683321654e-2 * t35585;
    let t35587 = t30374 * t8606;
    let t35594 = t7426 * t8480 * t7569;
    let t35595 = 0.42874018118069736972e-3 * t35594;
    let t35596 = t7433 * t8481;
    let t35597 = 0.12862205435420921092e-2 * t35596;
    let t35601 = t34161 * t8465;
    let t35602 = 0.56606566121287473722e-1 * t35601;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    (t35581, t35586, t35587, t35595, t35597, t35602, t35608)
}
