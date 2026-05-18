//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1093/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1093<F: Float>(t8392: F, t9794: F, t9985: F, t10074: F, t10079: F, t10080: F, t10166: F, t1901: F, t1934: F, t2409: F, t242: F, t2459: F, t2599: F, t2600: F, t2606: F, t41403: F, t42819: F, t42832: F, t446: F, t684: F, t724: F, t9787: F, t9793: F, t9983: F) -> F {
    let t42834 = t8392 * t9794;
    let t42836 = t8392 * t9985;
    let t42850 = -F::new(4.0) / F::new(9.0) * t446 * t724 * t10166 * t684 - F::new(16.0) / F::new(27.0) * t42819 - F::new(8.0) * t446 * t242 * t41403 - F::new(8.0) / F::new(3.0) * t1901 * t9787 * t9793 + F::new(2.0) / F::new(3.0) * t1901 * t2599 * t2600 * t1934 * t2459 + F::new(8.0) / F::new(9.0) * t42832 + F::new(8.0) / F::new(9.0) * t42834 - F::new(4.0) / F::new(9.0) * t42836 - F::new(4.0) / F::new(3.0) * t1901 * t2599 * t9983 * t2409 + F::new(8.0) / F::new(3.0) * t1901 * t10079 * t10080 * t2409 + F::new(8.0) / F::new(3.0) * t1901 * t2606 * t10074 * t2409;
    t42850
}
