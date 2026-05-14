//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1020/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1020<F: Float>(t148750: F, t148765: F, t148781: F, t148797: F, t148814: F, t148831: F, t148844: F, t148856: F, t605: F, t12664: F, t33044: F, t104623: F, t1384: F, t35149: F, t604: F, t609: F) -> (F, F, F, F) {
    let t148860 = t605 * (t148750 + t148765 + t148781 + t148797 + t148814 + t148831 + t148844 + t148856);
    let t148880 = t12664 * t33044;
    let t148897 = t104623 * t1384;
    let t148905 = t35149 * t604;
    let t148906 = t148905 * t609;
    (t148860, t148880, t148897, t148906)
}
