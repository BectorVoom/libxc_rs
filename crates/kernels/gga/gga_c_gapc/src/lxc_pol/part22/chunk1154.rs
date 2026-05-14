//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1154/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1154<F: Float>(t35588: F, t35591: F, t35595: F, t35597: F, t35599: F, t35601: F, t35606: F, t35610: F, t35613: F, t35615: F, t35618: F, t35620: F, t35623: F, t36405: F, t35628: F, t35631: F) -> (F, F, F) {
    let t36419 = -0.12147342662753799615e-3 * t35588 + 0.2429468532550759923e-3 * t35591 - 0.643771148843624415e-7 * t35595 + 0.17379648562707520765e-4 * t35597 + 0.10527696974386626333e-2 * t35599 + 0.10527696974386626333e-2 * t35601 - 0.18314001642303427359e-5 * t35606 + 0.3090101514449397192e-4 * t35610 + 0.3090101514449397192e-4 * t35613 + 0.12147342662753799615e-3 * t35615 + 0.17379648562707520765e-4 * t35618 - 0.81105026625968430236e-4 * t35620 + 0.17379648562707520765e-3 * t35623;
    let t36420 = t36405 + t36419;
    let t36421 = 0.17379648562707520765e-3 * t35628;
    let t36422 = 0.86898242813537603825e-4 * t35631;
    (t36420, t36421, t36422)
}
