//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 960/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk960<F: Float>(t11513: F, t2664: F, t7294: F, t11325: F, t3363: F, t3789: F, t11808: F, t11983: F, t11772: F, t29692: F, t11795: F, t9387: F, t11508: F, t3402: F, t7944: F, t33536: F, t33541: F, t33547: F, t33552: F, t33555: F) -> (F, F) {
    let t33558 = t7294 * t11513 * t2664;
    let t33560 = t3363 * t11325;
    let t33561 = t33560 * t3789;
    let t33563 = t11808 * t11983;
    let t33565 = t11772 * t29692;
    let t33567 = t9387 * t11795;
    let t33570 = t3402 * t11508 * t7944;
    let t33572 = 0.2209855149968790001e-7 * t33536 - 0.26904388710304542825e-7 * t33541 + 0.2504163411376437654e-5 * t33547 - 0.44524025454273061491e-5 * t33552 - 0.30353495895471971564e-6 * t33555 + 0.53968515702149165444e-6 * t33558 - 0.32042899674547455014e-6 * t33561 - 0.32042899674547455014e-6 * t33563 + 0.63252766927083333336e-6 * t33565 + 0.27462095132499841011e-4 * t33567 + 0.30353495895471971564e-6 * t33570;
    (t33560, t33572)
}
