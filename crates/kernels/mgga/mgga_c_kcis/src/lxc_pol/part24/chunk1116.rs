//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1116/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1116<F: Float>(t100466: F, t100474: F, t100477: F, t100480: F, t100482: F, t100486: F, t100489: F, t27028: F, t27042: F, t28146: F, t28190: F, t29127: F, t5329: F, t68901: F, t7772: F, t7788: F) -> (F,) {
    let t100491 = -0.69505208333333333334e-3 * t7788 * t5329 * t27028 * t68901 - 0.34752604166666666667e-3 * t7788 * t100466 - 0.46377350260416666667e-4 * t7772 * t100466 - 0.46336805555555555556e-3 * t28190 * t28146 + 0.69644166666666666666e-2 * t100474 - 0.92858888888888888888e-2 * t100477 - 0.23214722222222222222e-2 * t100480 - 0.30945286961263020834e-5 * t100482 - 0.12367293402777777778e-3 * t27042 * t29127 + 0.12897067901234567901e-2 * t100486 - 0.51588271604938271605e-2 * t100489;
    (t100491,)
}
