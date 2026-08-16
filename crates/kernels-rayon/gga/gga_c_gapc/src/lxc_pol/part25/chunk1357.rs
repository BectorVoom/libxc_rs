//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1357/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1357(t35503: f64, t35506: f64, t35510: f64, t35512: f64, t35515: f64, t35519: f64, t35521: f64, t35524: f64, t35531: f64, t35533: f64, t35539: f64, t35543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36373 = 0.5061392776147416506e-5_f64 * t35503;
    let t36375 = 0.2530696388073708253e-5_f64 * t35506;
    let t36376 = 0.14762395597096631476e-5_f64 * t35510;
    let t36377 = 0.17379648562707520765e-3_f64 * t35512;
    let t36378 = 0.10862280351692200478e-4_f64 * t35515;
    let t36379 = 0.10862280351692200478e-4_f64 * t35519;
    let t36380 = 0.11948508386861420526e-3_f64 * t35521;
    let t36381 = 0.10862280351692200478e-4_f64 * t35524;
    let t36383 = 0.64377114884362441502e-6_f64 * t35531;
    let t36384 = 0.20020620314538669735e-3_f64 * t35533;
    let t36386 = 0.64377114884362441502e-6_f64 * t35539;
    let t36387 = 0.14082493880954284079e-6_f64 * t35543;
    (t36373, t36375, t36376, t36377, t36378, t36379, t36380, t36381, t36383, t36384, t36386, t36387)
}
