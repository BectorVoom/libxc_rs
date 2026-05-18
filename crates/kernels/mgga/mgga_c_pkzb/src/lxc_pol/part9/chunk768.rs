//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 768/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk768<F: Float>(t5462: F, t626: F, t1784: F, t1792: F, t1813: F, t184: F, t188: F, t5408: F, t5420: F, t5424: F, t622: F, t634: F) -> (F, F) {
    let t5463 = t626 * t5462;
    let t5466 = F::new(0.65854491829355115987e0) * t5408 * t188 - F::new(0.19756347548806534796e1) * t1784 * t634 + F::new(0.39512695097613069591e1) * t622 * t1792 - F::new(0.19756347548806534796e1) * t622 * t1813 - F::new(0.39512695097613069591e1) * t184 * t5420 + F::new(0.39512695097613069591e1) * t184 * t5424 - F::new(0.65854491829355115987e0) * t184 * t5463;
    (t5463, t5466)
}
