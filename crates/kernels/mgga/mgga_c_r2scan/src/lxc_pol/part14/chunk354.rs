//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 354/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk354<F: Float>(t44: F, t51: F, t406: F, t425: F, t458: F, t99: F, t1213: F, t1219: F, t48: F, t101: F, t1225: F, t1228: F, t53: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t1357 = t406 * t425;
    let t1358 = F::new(8.0) * t1357;
    let t1359 = t406 * t458;
    let t1360 = F::new(8.0) * t1359;
    let t1361 = F::new(1.0) / t99;
    let t1367 = piecewise3::<F>(t45, F::new(0.0), F::new(4.0) / F::new(9.0) * t1361 * t1213 + F::new(4.0) / F::new(3.0) * t48 * t1219);
    let t1368 = F::new(1.0) / t101;
    let t1374 = piecewise3::<F>(t52, F::new(0.0), F::new(4.0) / F::new(9.0) * t1368 * t1225 + F::new(4.0) / F::new(3.0) * t53 * t1228);
    let t1375 = t1367 + t1374;
    (t1357, t1358, t1360, t1361, t1368, t1375)
}
