//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 307/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk307<F: Float>(t350: F, t55: F, t95: F, t367: F, t4: F, t44: F, t382: F, t79: F, t373: F, t51: F, t379: F, t381: F) -> (F, F, F, F, F, F, F) {
    let t1216 = t350 * t95 * t55;
    let t1218 = F::cast_from(0.24415406715670879921e-3_f64) * t367 * t1216;
    let t1219 = t44 * t4;
    let t1220 = t79 * t382;
    let t1222 = F::cast_from(0.10843580882781524214e-1_f64) * t1219 * t1220;
    let t1223 = t373 * t51;
    let t1224 = F::cast_from(1.0_f64) / t1223;
    let t1225 = t379 * t379;
    let t1227 = t1224 * t1225 * t381;
    (t1216, t1218, t1220, t1222, t1224, t1225, t1227)
}
