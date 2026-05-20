//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1112/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1112<F: Float>(t34790: F, t34795: F, t34800: F, t34827: F, t3: F, t1918: F, t2115: F, t2170: F, t34011: F, t34014: F, t34346: F, t34348: F, t34350: F, t34358: F, t34362: F, t34365: F, t34368: F, t573: F, t8124: F, t8127: F, t8245: F, t8616: F, t8905: F, param_d: F) -> (F, F, F, F) {
    let t34829 = t34790 + t34795 + t34800 + t34827;
    let t34830 = t3 * t34829;
    let t34838 = param_d * t34829;
    let t34848 = F::new(3.0) * t1918 * t8905 + F::new(3.0) * t2115 * t8245 + F::new(6.0) * t2170 * t8124 + F::new(3.0) * t2170 * t8127 + t34838 * t573 + t34011 + t34014 + t34346 + t34348 + t34350 + t34358 + t34362 + t34365 + t34368 + t8616;
    (t34829, t34830, t34838, t34848)
}
