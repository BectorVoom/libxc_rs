//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1414/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1414<F: Float>(t209: F, t36420: F, t36449: F, t38539: F, t38542: F, t38545: F, t38548: F, t38552: F, t34361: F, t34373: F, t36906: F, t36907: F, t36908: F, t36909: F, t36910: F, t36911: F, t36913: F, t36914: F, t36915: F) -> (F, F) {
    let t38556 = (t38539 + t38542 + t38545 + t38548 + t36420 + t38552 + t36449) * t209;
    let t38565 = t36906 + t36907 - t36908 - t36909 + t36910 + t36911 - F::new(0.56912804804009946682e-7) * t34361 + t36913 + t36914 - t36915 + F::new(0.68360384691762319208e-5) * t34373;
    (t38556, t38565)
}
