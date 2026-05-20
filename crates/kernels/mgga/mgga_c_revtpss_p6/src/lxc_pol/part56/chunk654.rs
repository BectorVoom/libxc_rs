//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 654/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk654<F: Float>(t33: F, t775: F, t890: F, t1113: F, t1940: F, t1963: F, t2403: F, t7087: F, t7091: F, t1936: F, t2322: F, t5523: F) -> (F, F, F, F, F) {
    let t7200 = t33 * t775;
    let t7207 = t33 * t890;
    let t7214 = F::new(3.0) / F::new(2.0) * t2403 * t1963 * t7200 + t1940 * t7087 * t33 / F::new(2.0) - t1940 * t7091 * t7207 / F::new(2.0) + t1940 * t1963 * t1113 / F::new(2.0);
    let t7226 = F::new(2.0) * t2322 * t1936;
    let t7228 = F::new(2.0) * t5523 * t1936;
    (t7200, t7207, t7214, t7226, t7228)
}
