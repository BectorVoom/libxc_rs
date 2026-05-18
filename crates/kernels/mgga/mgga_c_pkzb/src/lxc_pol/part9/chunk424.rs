//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 424/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk424<F: Float>(t12: F, t24: F, t1643: F, t1646: F, t207: F, t652: F, t1652: F, t1655: F, t333: F, t821: F, zeta_threshold: F) -> F {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t1684 = piecewise3::<f64>(t84, F::new(0.0), -F::new(2.0) / F::new(9.0) * t652 * t1643 + F::new(2.0) / F::new(3.0) * t207 * t1646);
    let t1690 = piecewise3::<f64>(t90, F::new(0.0), -F::new(2.0) / F::new(9.0) * t821 * t1652 + F::new(2.0) / F::new(3.0) * t333 * t1655);
    let t1692 = t1684 / F::new(2.0) + t1690 / F::new(2.0);
    t1692
}
