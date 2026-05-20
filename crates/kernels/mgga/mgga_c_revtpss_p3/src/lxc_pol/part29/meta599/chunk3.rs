//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2042/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2042<F: Float>(t10416: F, t13435: F, t13521: F, t13648: F, t1518: F, t18227: F, t2014: F, t2056: F, t2107: F, t2322: F, t25082: F, t26154: F, t26674: F, t26679: F, t27123: F, t27126: F, t27833: F, t28286: F, t28588: F, t28760: F, t28932: F, t28935: F, t49564: F, t651: F, t7235: F, t7359: F, t7367: F, t7374: F, t7536: F, t7537: F, t75485: F, t7732: F, t7898: F, t7978: F, t95088: F, t97654: F, t98535: F) -> F {
    let t103999 = -F::new(2.0) * t651 * t26674 * t1518 - F::new(2.0) * t75485 * t2056 - F::new(4.0) * t18227 * t7367 - F::new(6.0) * t95088 * t28588 - F::new(2.0) * t7732 * t26154 - F::new(2.0) * t10416 * t7978 - F::new(4.0) * t13435 * t7978 - F::new(4.0) * t2322 * t28760 - F::new(4.0) * t27123 * t7374 - F::new(2.0) * t98535 * t2056 - F::new(4.0) * t27126 * t7367 + F::new(2.0) * t7898 * t26679 + F::new(2.0) * t27833 * t7537 - F::new(2.0) * t7359 * t13521 + F::new(6.0) * t7235 * t28935 + F::new(6.0) * t7235 * t28932 + F::new(12.0) * t25082 * t28286 * t97654 - t2014 * t2107 * t49564 - F::new(2.0) * t2014 * t7536 * t13648;
    t103999
}
