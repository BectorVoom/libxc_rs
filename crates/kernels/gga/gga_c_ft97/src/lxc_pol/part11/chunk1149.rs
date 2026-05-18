//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1149/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1149<F: Float>(t10801: F, t1882: F, t10703: F, t10712: F, t10722: F, t15312: F, t1901: F, t2409: F, t2894: F, t296: F, t43332: F, t44057: F, t44131: F, t44135: F, t44145: F, t44147: F, t44149: F, t446: F, t684: F, t835: F) -> F {
    let t44151 = t1882 * t10801;
    let t44153 = -F::new(8.0) / F::new(9.0) * t44057 + F::new(4.0) / F::new(3.0) * t446 * t835 * t2894 * t2409 + F::new(2.0) * t446 * t296 * t43332 - t446 * t296 * t44131 / F::new(3.0) + F::new(8.0) / F::new(9.0) * t44135 - F::new(8.0) / F::new(3.0) * t1901 * t15312 * t10712 * t684 - F::new(4.0) / F::new(3.0) * t1901 * t10703 * t10722 * t684 + F::new(16.0) / F::new(9.0) * t44145 + F::new(8.0) / F::new(3.0) * t44147 + F::new(4.0) / F::new(3.0) * t44149 + F::new(4.0) / F::new(9.0) * t44151;
    t44153
}
