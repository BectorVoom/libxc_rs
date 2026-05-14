//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 992/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk992<F: Float>(t2089: F, t2127: F, t2163: F, t34255: F, t34260: F, t34263: F, t34265: F, t34267: F, t34268: F, t34271: F, t34285: F, t34294: F, t34300: F, t34304: F, t7969: F, t8065: F, t8152: F) -> (F,) {
    let t34800 = -t2089 * t8152 - t2127 * t8065 - t2163 * t7969 - t34255 - t34260 - t34263 - t34265 - t34267 - t34268 + t34271 - t34285 - t34294 + t34300 - t34304;
    (t34800,)
}
