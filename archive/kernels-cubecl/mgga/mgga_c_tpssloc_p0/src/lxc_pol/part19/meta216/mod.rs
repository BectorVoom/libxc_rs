//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk913;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta216<F: Float>(t2840: F, t287: F, t275: F, t2793: F, t912: F, t2844: F, t10294: F, t10544: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10547: F, t10550: F, t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F) -> (F, F, F, F, F, F, F) {
        let (t10660, t10661, t10662, t10663, t10665, t10680) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk913::<F>(t2840, t287, t275, t2793, t912, t2844, t10294, t10544, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10547, t10550);
        let t10695 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk914::<F>(t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589, t10591, t10597, t10600);
    (t10660, t10661, t10662, t10663, t10665, t10680, t10695)
}
