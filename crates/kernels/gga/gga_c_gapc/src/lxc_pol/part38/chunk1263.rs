//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1263/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1263<F: Float>(t3635: F, t8521: F, t11198: F, t1928: F, t2903: F, t11199: F, t8422: F, t11223: F, t11257: F, t1577: F, t35588: F, t35591: F, t35595: F, t35597: F, t35599: F, t35601: F, t35606: F, t35610: F, t35613: F) -> F {
    let t35615 = t8521 * t3635;
    let t35618 = t2903 * t11198 * t1928;
    let t35620 = t8422 * t11199;
    let t35623 = t11257 * t11223 * t1577;
    let t35625 = -F::new(0.60736713313768998074e-4) * t35588 + F::new(0.12147342662753799615e-3) * t35591 - F::new(0.3218855744218122075e-7) * t35595 + F::new(0.86898242813537603824e-5) * t35597 + F::new(0.52638484871933131664e-3) * t35599 + F::new(0.52638484871933131664e-3) * t35601 - F::new(0.91570008211517136795e-6) * t35606 + F::new(0.1545050757224698596e-4) * t35610 + F::new(0.1545050757224698596e-4) * t35613 + F::new(0.60736713313768998074e-4) * t35615 + F::new(0.86898242813537603824e-5) * t35618 - F::new(0.40552513312984215118e-4) * t35620 + F::new(0.86898242813537603824e-4) * t35623;
    t35625
}
