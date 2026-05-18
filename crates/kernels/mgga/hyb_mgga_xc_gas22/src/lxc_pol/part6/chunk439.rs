//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 439/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk439<F: Float>(t43: F, t1946: F, t1947: F, t1990: F, t616: F, t635: F, t72: F, t88: F, t29: F, t125: F, t26: F, t1796: F) -> (F, F, F, F) {
    let t44 = F::new(0.135e1) <= t43;
    let t1994 = piecewise3::<f64>(t44, t1946, -F::new(8.0) / F::new(3.0) * t1947 * t88 - F::new(16.0) / F::new(3.0) * t616 * t635 - F::new(8.0) / F::new(3.0) * t72 * t1990);
    let t1995 = t29 * t1994;
    let t1996 = t1995 * t125;
    let t1997 = t26 * t1996;
    let t2002 = -t1796;
    (t1994, t1996, t1997, t2002)
}
