//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1177/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1177(t34659: f64, t30661: f64, t30673: f64, t30675: f64, t30690: f64, t30695: f64, t30705: f64, t30709: f64, t32543: f64, t32544: f64, t32545: f64, t34657: f64, t34663: f64, t34667: f64, t34671: f64, t34675: f64, t34684: f64, t34686: f64) -> f64 {
    let t37197 = 7.0_f64 / 36.0_f64 * t34659;
    let t37208 = 0.80031500487063509014e-2_f64 * t30661 - t32543 - t32544 + t32545 - 0.68598428988911579156e-2_f64 * t30673 - 0.34299214494455789578e-2_f64 * t30675 - t34657 / 48.0_f64 + t37197 + 0.62896184579208304138e-3_f64 * t34663 + 0.12862205435420921092e-2_f64 * t34667 + 0.94344276868812456204e-2_f64 * t34671 + 0.83861579438944405518e-3_f64 * t34675 - 0.68598428988911579156e-2_f64 * t30690 + 0.14291339372689912324e-2_f64 * t30695 - 0.2096539485973610138e-2_f64 * t30705 - 0.12579236915841660827e-2_f64 * t30709 - 0.12862205435420921092e-2_f64 * t34684 - t34686 / 24.0_f64;
    t37208
}
