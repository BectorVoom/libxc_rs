//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 444/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk444<F: Float>(t1812: F, t626: F, t1784: F, t1792: F, t184: F, t188: F, t622: F, t634: F) -> (F, F) {
    let t1813 = t626 * t1812;
    let t1816 = F::new(0.65854491829355115987e0) * t1784 * t188 - F::new(0.13170898365871023197e1) * t622 * t634 + F::new(0.13170898365871023197e1) * t184 * t1792 - F::new(0.65854491829355115987e0) * t184 * t1813;
    (t1813, t1816)
}
