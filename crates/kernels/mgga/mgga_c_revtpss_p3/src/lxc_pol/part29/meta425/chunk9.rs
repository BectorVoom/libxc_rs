//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1573/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1573<F: Float>(t12252: F, t12261: F, t12263: F, t12265: F, t12459: F, t12460: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F, t17061: F, t17066: F, t17083: F) -> F {
    let t17085 = F::cast_from(0.23154444444444444444e-1_f64) * t12252 + F::cast_from(0.23154444444444444444e0_f64) * t12261 - F::cast_from(0.69463333333333333333e-1_f64) * t12263 - F::cast_from(0.13892666666666666667e0_f64) * t12265 + F::cast_from(0.264729375e1_f64) * t16852 - F::cast_from(0.157790625e0_f64) * t16855 - F::cast_from(0.3529725e1_f64) * t16858 - F::cast_from(0.17648625e1_f64) * t16860 + F::cast_from(0.6311625e0_f64) * t16863 + F::cast_from(0.31558125e0_f64) * t16865 + t17061 - F::cast_from(0.34431666666666666667e0_f64) * t16731 + F::cast_from(0.20839e0_f64) * t16887 + F::cast_from(0.62517e0_f64) * t16890 - t17066 - F::cast_from(0.69463333333333333334e-1_f64) * t16895 - t12459 - t12460 - F::cast_from(0.34731666666666666667e-1_f64) * t16898 - F::cast_from(0.20839e0_f64) * t16901 + F::cast_from(0.41678e0_f64) * t16904 + t17083;
    t17085
}
