//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1358/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1358<F: Float>(t35503: F, t35506: F, t35510: F, t35512: F, t35515: F, t35519: F, t35521: F, t35524: F, t35531: F, t35533: F, t35539: F, t35543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36373 = F::cast_from(0.5061392776147416506e-5_f64) * t35503;
    let t36375 = F::cast_from(0.2530696388073708253e-5_f64) * t35506;
    let t36376 = F::cast_from(0.14762395597096631476e-5_f64) * t35510;
    let t36377 = F::cast_from(0.17379648562707520765e-3_f64) * t35512;
    let t36378 = F::cast_from(0.10862280351692200478e-4_f64) * t35515;
    let t36379 = F::cast_from(0.10862280351692200478e-4_f64) * t35519;
    let t36380 = F::cast_from(0.11948508386861420526e-3_f64) * t35521;
    let t36381 = F::cast_from(0.10862280351692200478e-4_f64) * t35524;
    let t36383 = F::cast_from(0.64377114884362441502e-6_f64) * t35531;
    let t36384 = F::cast_from(0.20020620314538669735e-3_f64) * t35533;
    let t36386 = F::cast_from(0.64377114884362441502e-6_f64) * t35539;
    let t36387 = F::cast_from(0.14082493880954284079e-6_f64) * t35543;
    (t36373, t36375, t36376, t36377, t36378, t36379, t36380, t36381, t36383, t36384, t36386, t36387)
}
