//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 307/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk307<F: Float>(t7: F, t132: F, t1057: F, t496: F, t224: F, t545: F, t341: F, t675: F, t259: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t1059 = F::new(4.0) * t1057 * t496;
    let t1062 = piecewise3::<f64>(t8, F::new(0.0), F::new(4.0) / F::new(3.0) * t224 * t545);
    let t1065 = piecewise3::<f64>(t133, F::new(0.0), F::new(4.0) / F::new(3.0) * t341 * t675);
    let t1067 = (t1062 + t1065) * t259;
    (t1059, t1067)
}
