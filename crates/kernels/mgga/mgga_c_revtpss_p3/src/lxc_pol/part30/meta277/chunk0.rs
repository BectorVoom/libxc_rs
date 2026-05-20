//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1225/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1225<F: Float>(t1583: F, t1940: F, t198: F, t2403: F, t7091: F, t7847: F, t7850: F, t892: F, t1544: F, t33: F, t1963: F, t1711: F, t7783: F) -> (F, F, F, F, F) {
    let t7855 = -t1583 * t1940 * t7091 + t198 * t7850 * t892 + F::new(3.0) * t2403 * t7847;
    let t7862 = t33 * t1544;
    let t7863 = t1963 * t7862;
    let t7869 = t33 * t1583;
    let t7876 = F::new(3.0) / F::new(2.0) * t2403 * t7863 + t1940 * t7783 * t33 / F::new(2.0) - t1940 * t7091 * t7869 / F::new(2.0) + t1940 * t1963 * t1711 / F::new(2.0);
    (t7855, t7862, t7863, t7869, t7876)
}
