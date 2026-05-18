//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 496/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk496<F: Float>(t44: F, t51: F, t1213: F, t1219: F, t472: F, t99: F, t101: F, t1225: F, t1228: F, t476: F, zeta_threshold: F) -> F {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2244 = piecewise3::<f64>(t45, F::new(0.0), F::new(10.0) / F::new(9.0) * t472 * t1213 + F::new(5.0) / F::new(3.0) * t99 * t1219);
    let t2250 = piecewise3::<f64>(t52, F::new(0.0), F::new(10.0) / F::new(9.0) * t476 * t1225 + F::new(5.0) / F::new(3.0) * t101 * t1228);
    let t2252 = t2244 / F::new(2.0) + t2250 / F::new(2.0);
    t2252
}
