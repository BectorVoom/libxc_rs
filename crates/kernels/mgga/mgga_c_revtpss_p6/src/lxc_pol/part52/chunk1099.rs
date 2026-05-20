//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1099/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1099<F: Float>(t2035: F, t34270: F, t2007: F, t2052: F, t2056: F, t28030: F, t34250: F, t34253: F, t34255: F, t34260: F, t34263: F, t34265: F, t34267: F, t34268: F, t6985: F, t7883: F, t7969: F, t7984: F, t8463: F) -> F {
    let t34271 = t34270 * t2035;
    let t34274 = -t2007 * t7969 - t2052 * t7883 - F::new(2.0) * t2056 * t28030 - F::new(2.0) * t6985 * t7984 - t34250 - t34253 - t34255 - t34260 - t34263 - t34265 - t34267 - t34268 + t34271 - t8463;
    t34274
}
