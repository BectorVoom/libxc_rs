//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1373/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1373<F: Float>(t30763: F, t31542: F, t811: F, t30760: F, t816: F, t820: F, t1701: F, t19230: F, t6027: F, t27494: F, t4125: F, t112223: F, t112266: F, t112367: F, t127410: F, t127504: F, t127553: F, t127557: F, t1472: F, t14742: F, t19039: F, t19135: F, t2691: F, t28676: F, t4089: F, t4094: F, t4099: F, t4113: F, t4115: F, t5264: F, t6233: F, t6979: F, t82822: F) -> (F, F, F) {
    let t127577 = t31542 * t30763 * t811;
    let t127580 = t816 * t30760;
    let t127582 = t127580 * t30763 * t820;
    let t127591 = t1701 * t6027 * t19230;
    let t127595 = t1701 * t27494 * t4125;
    let t127609 = 0.10947790369858991997e1 * t19135 * t127504 + 0.15303647250623035441e2 * t4094 * t127553 - 0.76518236253115177207e1 * t4099 * t127557 - 0.81472461409953017306e-1 * t2691 * t127577 + 0.40736230704976508653e-1 * t4113 * t127582 + 0.24441738422985905192e0 * t28676 * t127577 - 0.22226000364197530865e-1 * t112266 + 0.90613700826057446696e0 * t14742 * t127410 - 0.22653425206514361674e0 * t4099 * t127591 + 0.24163653553615319118e1 * t4099 * t127595 - 0.10947790369858991998e1 * t19039 * t6979 * t4089 - 0.10947790369858991998e1 * t5264 * t112223 * t112367 * t4115 + 0.22653425206514361674e0 * t1472 * t127591 - 0.45306850413028723348e0 * t82822 * t6233;
    (t127582, t127595, t127609)
}
