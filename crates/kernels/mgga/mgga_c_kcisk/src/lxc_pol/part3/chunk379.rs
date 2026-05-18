//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 379/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk379<F: Float>(t1757: F, t1899: F, t1800: F, t1869: F, t1689: F, t1693: F, t1792: F, t1796: F, t1804: F, t1866: F, t1897: F, t671: F) -> (F, F, F, F) {
    let t1900 = t1899 * t1757;
    let t1901 = t1800 * t1900;
    let t1902 = t1869 * t1901;
    let t1904 = t1689 * t671 - F::new(0.193e0) * t1693 * t1792 + t1796 + F::new(0.16581944444444444444e-2) * t1804 + F::new(0.24872916666666666666e-2) * t1866 - F::new(0.24872916666666666666e-2) * t1897 + F::new(0.16581944444444444444e-2) * t1902;
    (t1900, t1901, t1902, t1904)
}
