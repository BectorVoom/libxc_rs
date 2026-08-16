//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 924/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk924<F: Float>(t13998: F, t14571: F, t14574: F, t14577: F, t14579: F, t14583: F, t14587: F, t14591: F, t14592: F, t14593: F, t14594: F, t14595: F, t14596: F, t14597: F, t14598: F, t14599: F) -> F {
    let t76594 = t14571 - t14574 + t14577 + t14579 - t14583 + t14587 + t14591 + t14592 + t14593 - t14594 + t14595 + t13998 + t14596 + t14597 + t14598 + t14599;
    t76594
}
