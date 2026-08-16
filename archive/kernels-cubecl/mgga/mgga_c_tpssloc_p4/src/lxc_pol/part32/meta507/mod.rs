//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta507<F: Float>(t254: F, t563: F, t12020: F, t2015: F, t5325: F, t1323: F, t7722: F, t1827: F, t22765: F, t5234: F, t6944: F, t1354: F) -> (F, F, F, F, F, F, F) {
        let (t26224, t26225, t26226, t26229, t26231, t26233, t26234) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1833::<F>(t254, t563, t12020, t2015, t5325, t1323, t7722, t1827, t22765, t5234, t6944, t1354);
    (t26224, t26225, t26226, t26229, t26231, t26233, t26234)
}
