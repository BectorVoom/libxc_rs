//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1331/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1331<F: Float>(t10314: F, t20592: F, t6710: F, t34246: F, t6717: F, t6914: F, t10513: F, t20441: F, t10532: F, t4529: F, t579: F, t30903: F) -> (F, F, F, F, F) {
    let t34749 = F::new(0.30674340763136599742e2) * t6710 * t20592 * t10314;
    let t34752 = F::new(0.62115540045351614476e2) * t6914 * t6717 * t34246;
    let t34762 = F::new(0.1656414401209376386e3) * t6914 * t20441 * t10513;
    let t34766 = F::new(0.73618417831527839379e2) * t10532 * t579 * t4529 * t10513;
    let t34773 = F::new(0.63904876589867916128e-1) * t30903;
    (t34749, t34752, t34762, t34766, t34773)
}
