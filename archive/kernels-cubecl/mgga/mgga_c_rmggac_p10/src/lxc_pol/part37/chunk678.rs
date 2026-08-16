//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 678/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk678<F: Float>(t14132: F, t68541: F, t14251: F, t68524: F, t14162: F, t7254: F, t1986: F, t2092: F, t24983: F, t3129: F, t14046: F, t14367: F) -> (F, F, F, F, F, F) {
    let t68542 = t68541 * t14132;
    let t68543 = F::cast_from(0.16351352353374609375e-5_f64) * t68542;
    let t68549 = t68524 * t14251;
    let t68550 = F::cast_from(0.11634323970834742769e-3_f64) * t68549;
    let t68552 = t7254 * t14162;
    let t68555 = t1986 * t2092;
    let t68575 = F::cast_from(1.0_f64) / t3129 / t24983;
    let t68581 = t14046 * t14367;
    (t68543, t68550, t68552, t68555, t68575, t68581)
}
