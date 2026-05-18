//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 995/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk995<F: Float>(t33: F, t7782: F, t1711: F, t1940: F, t2403: F, t31863: F, t31876: F, t33727: F, t7091: F, t7862: F, t7869: F, t8490: F, t8494: F) -> (F, F) {
    let t33888 = t33 * t7782;
    let t33896 = F::new(3.0) / F::new(2.0) * t2403 * t8490 * t7862 + t1940 * t33727 * t33 / F::new(2.0) - t1940 * t31863 * t7869 / F::new(2.0) + t1940 * t8490 * t1711 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2403 * t8494 * t7862 - t1940 * t7091 * t33888 + t1940 * t31876 * t7869 - t1940 * t8494 * t1711 / F::new(2.0);
    (t33888, t33896)
}
