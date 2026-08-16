//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1343/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1343<F: Float>(t22470: F, t28629: F, t2050: F, t27543: F, t5905: F, t22640: F, t27544: F, t22430: F, t28624: F, t22381: F, t22348: F, t5916: F, t97801: F) -> (F, F, F, F, F, F, F) {
    let t102975 = t28629 * t22470;
    let t102978 = t2050 * t27543 * t5905;
    let t102980 = t27544 * t22640;
    let t102982 = t28624 * t22430;
    let t102985 = t27544 * t22381;
    let t102987 = t28624 * t22348;
    let t102989 = t97801 * t5916;
    (t102975, t102978, t102980, t102982, t102985, t102987, t102989)
}
