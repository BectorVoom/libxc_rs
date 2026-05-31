//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 319/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk319<F: Float>(t174: F, t1650: F, t176: F, t1649: F, t44: F, t487: F, t447: F, t531: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t1653 = piecewise3::<F>(t175, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t176 * t1650);
    let t1655 = (t1649 + t1653) * t44;
    let t1880 = F::cast_from(1.0_f64) / t487;
    let t1881 = sigma2 * t1880;
    let t1884 = piecewise3::<F>(t175, F::cast_from(0.0_f64), t1650);
    let t1885 = t447 * t1884;
    let t1889 = t531 * t1650;
    (t1655, t1880, t1881, t1884, t1885, t1889)
}
