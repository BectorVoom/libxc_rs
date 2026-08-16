//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 978/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk978<F: Float>(t12916: F, t8947: F, t8946: F, t33494: F, t8938: F) -> (F, F, F) {
    let t33521 = t8947 * t12916;
    let t33523 = F::cast_from(0.12395776403017003607e-3_f64) * t8946 * t33521;
    let t33524 = t8938 * t33494;
    (t33521, t33523, t33524)
}
