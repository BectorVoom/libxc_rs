//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 512/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk512<F: Float>(t141: F, t4622: F, t4579: F, t930: F, t4583: F, t2848: F, t2892: F, t2905: F, t2906: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F) -> (F, F, F, F) {
    let t4623 = t141 * t4622;
    let t4625 = t930 * t4579;
    let t4626 = t141 * t4625;
    let t4628 = t930 * t4583;
    let t4629 = t141 * t4628;
    let t4631 = -0.9494625e0 * t4599 + 0.1898925e1 * t4607 + t2892 + 0.99655555555555555557e-1 * t2848 + 0.99655555555555555557e-1 * t4571 - 0.19931111111111111111e0 * t4576 + 0.59793333333333333334e0 * t4581 - 0.29896666666666666667e0 * t4585 + 0.15358125e0 * t4615 + 0.3071625e0 * t4617 + t2905 + 0.54771111111111111111e-1 * t2906 + 0.54771111111111111111e-1 * t4620 - 0.27385555555555555556e-1 * t4623 + 0.16431333333333333333e0 * t4626 - 0.82156666666666666667e-1 * t4629;
    (t4623, t4626, t4629, t4631)
}
