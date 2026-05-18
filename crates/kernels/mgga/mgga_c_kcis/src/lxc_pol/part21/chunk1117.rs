//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1117/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1117<F: Float>(t7780: F, t7784: F, t20: F, t251: F, t2865: F, t1240: F, t27055: F, t7788: F, t3611: F, t7794: F, t5329: F, t2197: F, t26841: F, t26844: F, t26846: F, t26849: F, t26852: F, t26966: F, t26977: F, t27014: F, t27070: F, t27077: F, t7775: F, t7796: F) -> (F, F, F, F, F, F, F) {
    let t27080 = t7780 * t7784;
    let t27083 = t251 * t2865 * t20;
    let t27084 = t1240 * t27083;
    let t27087 = t7788 * t27055;
    let t27089 = t7794 * t3611;
    let t27090 = t5329 * t27089;
    let t27093 = -F::new(0.61905925925925925925e-2) * t26841 + F::new(0.11607361111111111111e-2) * t26844 - F::new(0.23214722222222222222e-2) * t26846 + F::new(0.23214722222222222222e-2) * t26849 - F::new(0.18534722222222222222e-2) * t26966 * t7796 - F::new(0.18534722222222222222e-2) * t26966 * t7775 + F::new(0.69505208333333333334e-3) * t27014 * t7775 + F::new(0.92754700520833333334e-4) * t27070 * t7775 - F::new(0.92858888888888888886e-2) * t26852 + F::new(0.69505208333333333334e-3) * t27014 * t7796 - F::new(0.92835860883789062501e-5) * t27077 * t26977 + F::new(0.61782407407407407408e-3) * t27080 - F::new(0.33980324074074074074e-2) * t27084 * t2197 + F::new(0.23168402777777777778e-3) * t27087 + F::new(0.34752604166666666667e-3) * t7788 * t27090;
    (t27080, t27083, t27084, t27087, t27089, t27090, t27093)
}
