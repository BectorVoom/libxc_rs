//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 672/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk672<F: Float>(t12461: F, t6324: F, t112: F, t6470: F, t107: F, t240: F, t625: F, t656: F, t2331: F, t63: F, t2267: F, t38: F) -> (F, F, F, F, F, F, F) {
    let t20085 = t6324 * t12461;
    let t20162 = t6470 * t112;
    let t22468 = t240 * t107;
    let t22469 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t22470 = t625 * t656;
    let t22473 = t63 * t2331;
    let t22505 = t38 * t2267;
    (t20085, t20162, t22468, t22469, t22470, t22473, t22505)
}
