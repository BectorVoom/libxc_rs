//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1270/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1270<F: Float>(t218: F, t675: F, t7984: F, t7988: F, t1180: F, t5555: F, t1878: F, t3061: F, t3065: F, t22233: F, t18427: F, t18430: F, t18433: F, t18468: F, t22230: F, t22236: F, t22262: F) -> (F, F, F, F, F, F, F, F) {
    let t22284 = t218 * t675 * t7984;
    let t22287 = t218 * t675 * t7988;
    let t22290 = t218 * t5555 * t1180;
    let t22293 = t218 * t1878 * t3061;
    let t22294 = F::new(0.82785e0) * t22293;
    let t22296 = t218 * t1878 * t3065;
    let t22297 = F::new(0.82785e0) * t22296;
    let t22302 = F::new(4.0) / F::new(3.0) * t22233;
    let t22303 = t18468 - F::new(28.0) / F::new(9.0) * t18427 + F::new(4.0) / F::new(3.0) * t18430 - t18433 / F::new(3.0) - F::new(28.0) / F::new(27.0) * t22230 + t22302 - t22236 + t22262;
    (t22284, t22287, t22290, t22293, t22294, t22296, t22297, t22303)
}
