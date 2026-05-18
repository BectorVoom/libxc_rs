//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1356/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1356<F: Float>(t10547: F, t6820: F, t204: F, t2476: F, t34411: F, t34407: F, t6710: F, t6711: F, t1429: F, t2365: F, t2366: F, t7861: F) -> (F, F, F, F) {
    let t35185 = F::new(0.25025342966295298669e1) * t10547 * t6820;
    let t35188 = F::new(0.46011511144704899612e1) * t2476 * t204 * t34411;
    let t35192 = F::new(0.23005755572352449806e2) * t6710 * t6711 * t34407;
    let t35198 = t1429 * t2365 * t2366 * t7861;
    (t35185, t35188, t35192, t35198)
}
