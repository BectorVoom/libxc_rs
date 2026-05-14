//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1091/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1091<F: Float>(t35588: F, t35591: F, t35595: F, t35597: F, t35599: F, t35601: F, t35606: F, t35610: F, t35613: F, t35615: F, t35618: F, t35620: F, t35623: F, t1006: F, t11223: F, t1603: F) -> (F, F) {
    let t35625 = -0.60736713313768998074e-4 * t35588 + 0.12147342662753799615e-3 * t35591 - 0.3218855744218122075e-7 * t35595 + 0.86898242813537603824e-5 * t35597 + 0.52638484871933131664e-3 * t35599 + 0.52638484871933131664e-3 * t35601 - 0.91570008211517136795e-6 * t35606 + 0.1545050757224698596e-4 * t35610 + 0.1545050757224698596e-4 * t35613 + 0.60736713313768998074e-4 * t35615 + 0.86898242813537603824e-5 * t35618 - 0.40552513312984215118e-4 * t35620 + 0.86898242813537603824e-4 * t35623;
    let t35628 = t1006 * t11223 * t1603;
    (t35625, t35628)
}
