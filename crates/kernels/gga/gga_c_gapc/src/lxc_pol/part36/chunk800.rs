//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 800/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk800<F: Float>(t9595: F, t9597: F, t9600: F, t9603: F, t9607: F, t9610: F, t9614: F, t9616: F, t9618: F, t9621: F, t9625: F, t9628: F, t9631: F, t9636: F, t9639: F, t9642: F, t9646: F, t9649: F, t9653: F, t9656: F, t9659: F, t9662: F, t9665: F, t9668: F, t9671: F, t9674: F) -> (F, F) {
    let t10886 = 0.57970906942607043472e-5 * t9595 - 0.57970906942607043472e-5 * t9597 + 0.86956360413910565208e-5 * t9600 - 0.12380169846338434109e-5 * t9603 + 0.10136107947527008247e-3 * t9607 - 0.34752370105806885418e-3 * t9610 - 0.34752370105806885418e-3 * t9614 - 0.24326659074064819793e-2 * t9616 + 0.84540905957968605064e-6 * t9618 - 0.27801896084645508334e-2 * t9621 + 0.20240885416666666668e-4 * t9625 + 0.10120442708333333334e-3 * t9628 + 0.10120442708333333334e-3 * t9631;
    let t10901 = -0.10120442708333333334e-4 * t9636 - 0.17376185052903442709e-3 * t9639 + 0.28960308421505737848e-5 * t9642 + 0.42233783114695867695e-6 * t9646 - 0.2318836277704281739e-4 * t9649 + 0.56273499301538336858e-8 * t9653 + 0.56273499301538336858e-8 * t9656 - 0.55603792169291016668e-2 * t9659 + 0.24326659074064819792e-2 * t9662 - 0.55603792169291016668e-2 * t9665 + 0.18550690221634253912e-3 * t9668 - 0.10005428175813516294e-7 * t9671 - 0.51584041026410142121e-5 * t9674;
    (t10886, t10901)
}
