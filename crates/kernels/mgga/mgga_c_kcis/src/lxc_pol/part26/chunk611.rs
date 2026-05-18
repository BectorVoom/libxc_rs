//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 611/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk611<F: Float>(t174: F, t176: F, t2641: F, t6281: F, t6284: F, t44: F, t6280: F, t1926: F, t447: F, t1650: F, t2011: F, t4171: F, t4170: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t6288 = piecewise3::<f64>(t175, F::new(0.0), F::new(4.0) / F::new(9.0) * t2641 * t6281 + F::new(4.0) / F::new(3.0) * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6887 = F::new(1.0) / t1926;
    let t6888 = sigma2 * t6887;
    let t6895 = piecewise3::<f64>(t175, F::new(0.0), t6284);
    let t6896 = t447 * t6895;
    let t6903 = t1650 * t2011;
    let t6904 = t4171 * t6903;
    let t6905 = t4170 * t6904;
    (t6290, t6887, t6888, t6895, t6896, t6904, t6905)
}
