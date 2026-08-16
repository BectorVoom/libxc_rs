//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 704/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk704<F: Float>(t1413: F, t1444: F, t1449: F, t2481: F, t430: F, t453: F, t459: F, t4769: F, t4772: F, t4773: F, t4776: F, t4779: F, t4823: F, t4828: F, t4829: F, t4832: F, t4856: F) -> F {
    let t4859 = F::cast_from(0.496875e-1_f64) * t2481 * t1444 - F::cast_from(0.99375e-1_f64) * t4769 * t459 + F::cast_from(0.298125e0_f64) * t4772 * t4773 - F::cast_from(0.99375e-1_f64) * t1413 * t4776 - F::cast_from(0.99375e-1_f64) * t1413 * t4779 + F::cast_from(0.165625e-1_f64) * t430 * t4823 - F::cast_from(0.19875e0_f64) * t4828 * t4829 + F::cast_from(0.1490625e0_f64) * t1449 * t4832 - F::cast_from(0.165625e-1_f64) * t453 * t4856;
    t4859
}
