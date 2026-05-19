//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 412/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk412<F: Float>(t146: F, t20: F, t2861: F, t2: F, t816: F, t952: F, t2864: F, t2867: F, t2869: F, t15: F, t2863: F, t2866: F, t818: F, t947: F) -> (F, F, F, F) {
    let t3092 = t2861 * t146 * t20;
    let t3096 = t816 * t952 * t2;
    let t3104 = -F::cast_from(0.44044444444444444445e-2_f64) * t2864 + F::cast_from(0.88088888888888888889e-2_f64) * t2867 + F::cast_from(0.55033333333333333333e-2_f64) * t2869;
    let t3107 = -t3092 * t2863 / F::new(18.0) - t3096 * t818 / F::new(6.0) + t947 * t2866 / F::new(9.0) + t15 * t3104 / F::new(2.0);
    (t3092, t3096, t3104, t3107)
}
