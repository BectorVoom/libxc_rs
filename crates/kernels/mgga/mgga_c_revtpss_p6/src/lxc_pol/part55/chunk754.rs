//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 754/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk754<F: Float>(t1711: F, t1940: F, t1963: F, t2403: F, t33: F, t7091: F, t7783: F, t7863: F, t7869: F, t1936: F, t4248: F, t1518: F, t93: F) -> (F, F, F) {
    let t7876 = F::new(3.0) / F::new(2.0) * t2403 * t7863 + t1940 * t7783 * t33 / F::new(2.0) - t1940 * t7091 * t7869 / F::new(2.0) + t1940 * t1963 * t1711 / F::new(2.0);
    let t7888 = F::new(2.0) * t4248 * t1936;
    let t7889 = t93 * t1518;
    (t7876, t7888, t7889)
}
