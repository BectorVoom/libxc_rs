//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 933/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk933<F: Float>(t35055: F, t35076: F, t35180: F, t35204: F, t35238: F, t35240: F, t35244: F, t35271: F, t35359: F, t35418: F, t35425: F, t35456: F, t35471: F, t35486: F, t35529: F, t35560: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37366 = 0.15724046144802076034e-2 * t35055;
    let t37375 = 77.0 / 288.0 * t35076;
    let t37426 = 0.21437009059034868486e-3 * t35180;
    let t37435 = 0.13976929906490734252e-1 * t35204;
    let t37446 = 0.21437009059034868486e-2 * t35238;
    let t37447 = 0.12862205435420921092e-1 * t35240;
    let t37449 = 0.85748036236139473944e-3 * t35244;
    let t37464 = 0.21437009059034868486e-3 * t35271;
    let t37504 = 0.39221875e0 * t35359;
    let t37538 = 0.66040993808168719343e-1 * t35418;
    let t37541 = 0.95275595817932748827e-2 * t35425;
    let t37559 = 0.21437009059034868486e-2 * t35456;
    let t37565 = 0.19055119163586549766e-2 * t35471;
    let t37570 = 0.25724410870841842184e-2 * t35486;
    let t37591 = 0.68598428988911579156e-2 * t35529;
    let t37610 = 35.0 / 216.0 * t35560;
    (t37366, t37375, t37426, t37435, t37446, t37447, t37449, t37464, t37504, t37538, t37541, t37559, t37565, t37570, t37591, t37610)
}
