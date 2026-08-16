//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1136/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1136<F: Float>(t7426: F, t7569: F, t8480: F, t7433: F, t8481: F, t4680: F, t8463: F, t8652: F, t34161: F, t8465: F, t31421: F, t1992: F, t7585: F, t7842: F, t8402: F) -> (F, F, F, F, F, F) {
    let t35594 = t7426 * t8480 * t7569;
    let t35595 = F::cast_from(0.42874018118069736972e-3_f64) * t35594;
    let t35596 = t7433 * t8481;
    let t35597 = F::cast_from(0.12862205435420921092e-2_f64) * t35596;
    let t35599 = t8463 * t4680 * t8652;
    let t35601 = t34161 * t8465;
    let t35602 = F::cast_from(0.56606566121287473722e-1_f64) * t35601;
    let t35603 = F::cast_from(0.22921875e-1_f64) * t31421;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    (t35595, t35597, t35599, t35602, t35603, t35608)
}
