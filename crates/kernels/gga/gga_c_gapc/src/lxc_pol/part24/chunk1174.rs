//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1174/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1174<F: Float>(t34607: F, t5218: F, t33273: F, t5260: F, t676: F, t11543: F, t8751: F, t11425: F, t3085: F, t3664: F, t8903: F, t3691: F, t8728: F) -> (F, F, F, F, F, F) {
    let t34608 = t34607 * t5218;
    let t34611 = t5260 * t33273 * t676;
    let t34613 = t11543 * t8751;
    let t34615 = t11425 * t3085;
    let t34617 = t3664 * t8903;
    let t34619 = t3691 * t8728;
    (t34608, t34611, t34613, t34615, t34617, t34619)
}
