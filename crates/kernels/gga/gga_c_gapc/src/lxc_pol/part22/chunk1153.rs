//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1153/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1153<F: Float>(t35515: F, t35519: F, t35521: F, t35524: F, t35531: F, t35533: F, t35539: F, t35543: F, t35545: F, t35527: F, t35536: F, t36375: F, t36376: F, t36377: F, t35552: F, t35555: F, t35557: F, t35559: F, t35562: F, t35564: F, t35566: F, t35570: F, t35572: F, t35575: F, t35578: F, t35580: F, t35584: F) -> (F, F) {
    let t36378 = 0.10862280351692200478e-4 * t35515;
    let t36379 = 0.10862280351692200478e-4 * t35519;
    let t36380 = 0.11948508386861420526e-3 * t35521;
    let t36381 = 0.10862280351692200478e-4 * t35524;
    let t36383 = 0.64377114884362441502e-6 * t35531;
    let t36384 = 0.20020620314538669735e-3 * t35533;
    let t36386 = 0.64377114884362441502e-6 * t35539;
    let t36387 = 0.14082493880954284079e-6 * t35543;
    let t36388 = 0.5061392776147416506e-5 * t35545;
    let t36389 = t36375 + t36376 - t36377 + t36378 - t36379 + t36380 - t36381 - 0.54311401758461002391e-5 * t35527 - t36383 - t36384 + 0.54311401758461002391e-5 * t35536 + t36386 + t36387 - t36388;
    let t36405 = 0.36207601172307334926e-6 * t35552 + 0.36207601172307334926e-6 * t35555 - 0.11948508386861420526e-3 * t35557 - 0.75106634031756181752e-5 * t35559 - 0.3090101514449397192e-4 * t35562 - 0.16027743791133485603e-4 * t35564 - 0.809822844183586641e-4 * t35566 + 0.809822844183586641e-4 * t35570 + 0.39141437468873354315e-3 * t35572 - 0.2429468532550759923e-3 * t35575 - 0.12147342662753799615e-3 * t35578 - 0.2429468532550759923e-3 * t35580 + 0.2429468532550759923e-3 * t35584;
    (t36389, t36405)
}
