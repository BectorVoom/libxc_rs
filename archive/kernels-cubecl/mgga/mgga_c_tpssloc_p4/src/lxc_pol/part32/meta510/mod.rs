//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1836;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta510<F: Float>(t1811: F, t22797: F, t22804: F, t7709: F, t1361: F, t1799: F, t22690: F, t22792: F, t5227: F, t6916: F, t1998: F, t236: F, t5187: F, t6926: F, t22784: F, t22795: F, t26255: F, t26258: F, t26260: F, t26262: F) -> (F, F, F, F, F, F) {
        let (t26266, t26268, t26271, t26272, t26274, t26277) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1836::<F>(t1811, t22797, t22804, t7709, t1361, t1799, t22690, t22792, t5227, t6916, t1998, t236, t5187);
        let t26280 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1837::<F>(t26277, t6926, t22784, t22795, t26255, t26258, t26260, t26262, t26266, t26268, t26272, t26274);
    (t26266, t26268, t26271, t26272, t26277, t26280)
}
