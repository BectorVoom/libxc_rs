//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1243/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1243<F: Float>(t4417: F, t538: F, t22515: F, t26611: F, t34871: F, t104727: F, t115583: F, t115588: F, t115592: F, t119046: F, t16762: F, t1742: F, t23701: F, t23711: F, t23715: F, t26692: F, t5570: F, t8833: F, t94401: F, t94514: F, t94821: F, t94823: F) -> (F, F) {
    let t119056 = t4417 * t538;
    let t119068 = t22515 * t34871 * t26611;
    let t119083 = 0.66678001092592592596e-1 * t23715 * t5570 * t1742 * t119056 + 0.13335600218518518519e0 * t26692 * t115583 + 0.53706137268299704367e-1 * t23711 * t115588 + 0.24163653553615319119e1 * t8833 * t119046 - 0.96671047082939467857e0 * t94514 * t119068 + 0.96671047082939467857e0 * t94401 * t119068 - 0.12002040196666666667e1 * t104727 * t5570 * t34871 * t16762 - 0.53706137268299704367e-1 * t23701 * t115588 - 0.51860667516460905352e-1 * t26692 * t115592 + 0.22226000364197530866e-1 * t94821 - 0.26853068634149852184e-1 * t94823;
    (t119056, t119083)
}
