//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1122/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1122<F: Float>(t23745: F, t92447: F, t542: F, t94607: F, t133: F, t5813: F, t5814: F, t92557: F, t5821: F, t92574: F, t92968: F, t48678: F, t5812: F, t23832: F, t94753: F, t23742: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94771 = t23745 * t92447;
    let t94785 = t542 * t94607;
    let t94788 = t133 * t94607;
    let t94821 = t5813 * t92557 * t5814;
    let t94823 = t5821 * t92574;
    let t94827 = t5821 * t92968;
    let t94829 = t48678 * t5812;
    let t94836 = t23832 * t94753;
    let t94852 = t23742 * t92447;
    (t94771, t94785, t94788, t94821, t94823, t94827, t94829, t94836, t94852)
}
