//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 315/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk315<F: Float>(t174: F, t1650: F, t176: F, t1649: F, t44: F, t487: F, sigma2: F, zeta_threshold: F) -> (F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t1653 = piecewise3::<f64>(t175, F::new(0.0), F::new(4.0) / F::new(3.0) * t176 * t1650);
    let t1655 = (t1649 + t1653) * t44;
    let t1880 = F::new(1.0) / t487;
    let t1881 = sigma2 * t1880;
    (t1655, t1880, t1881)
}
