//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 818/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk818<F: Float>(t51: F, t3007: F, t4920: F, t1224: F, t3010: F, t476: F, t8584: F, t1217: F, t2517: F, t419: F, t8615: F, zeta_threshold: F) -> F {
    let t52 = t51 <= zeta_threshold;
    let t8616 = t4920 * t3007;
    let t8621 = t1224 * t3010;
    let t8624 = t476 * t8584;
    let t8627 = piecewise3::<F>(t52, F::new(0.0), F::new(8.0) / F::new(27.0) * t8616 * t419 + F::new(8.0) / F::new(9.0) * t2517 * t1217 - F::new(2.0) / F::new(9.0) * t8621 * t419 + F::new(2.0) / F::new(3.0) * t8624);
    let t8629 = t8615 / F::new(2.0) + t8627 / F::new(2.0);
    t8629
}
