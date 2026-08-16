//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1072/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1072(t30543: f64, t8610: f64, t30934: f64, t8614: f64, t1181: f64, t4533: f64, t604: f64, t7575: f64, t7433: f64, t8522: f64, t8518: f64, t5012: f64, t7564: f64, t8600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34702 = t30543 * t8610;
    let t34703 = 0.12862205435420921092e-1_f64 * t34702;
    let t34704 = t30934 * t8614;
    let t34708 = t7575 * t1181 * t604 * t4533;
    let t34710 = t7433 * t8522;
    let t34711 = 0.12862205435420921092e-2_f64 * t34710;
    let t34712 = t7433 * t8518;
    let t34713 = 0.12862205435420921092e-2_f64 * t34712;
    let t34716 = t7564 * t1181 * t8600 * t5012;
    (t34703, t34704, t34708, t34711, t34713, t34716)
}
