//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 445/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk445<F: Float>(t2917: F, t2966: F, t961: F) -> (F, F, F, F) {
    let t3013 = F::cast_from(0.40256666666666666667e0_f64) * t2917;
    let t3020 = F::new(0.137975e0) * t2966;
    let t3030 = t961 * t961;
    let t3031 = F::new(1.0) / t3030;
    (t3013, t3020, t3030, t3031)
}
