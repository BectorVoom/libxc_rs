//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1342/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1342<F: Float>(t10520: F, t1407: F, t204: F, t2476: F, t34407: F, t10615: F, t30848: F, t34371: F, t6710: F, t6711: F, t34321: F, t6717: F, t6914: F) -> (F, F, F, F, F) {
    let t34959 = F::new(0.18404604457881959845e2) * t1407 * t10520;
    let t34962 = F::new(0.92023022289409799224e1) * t2476 * t204 * t34407;
    let t34964 = F::new(0.50050685932590597338e1) * t10615 * t30848;
    let t34967 = F::new(0.23005755572352449806e2) * t6710 * t6711 * t34371;
    let t34970 = F::new(0.12423108009070322895e3) * t6914 * t6717 * t34321;
    (t34959, t34962, t34964, t34967, t34970)
}
