//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1223/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1223<F: Float>(t12048: F, t1580: F, t30788: F, t30791: F, t30793: F, t30805: F, t34556: F, t34566: F, t34573: F, t34576: F, t34579: F, t34581: F, t34583: F, t34586: F, t34588: F, t34592: F, t34595: F) -> (F,) {
    let t38648 = t34556 + 0.23005755572352449806e2 * t1580 * t12048 + t30788 + t30791 - 0.53964118009221795842e0 * t30793 - t34566 - t34573 + t34576 + t34579 + t34581 - t34583 - t34586 + t34588 - t34592 + t34595 + t30805;
    (t38648,)
}
