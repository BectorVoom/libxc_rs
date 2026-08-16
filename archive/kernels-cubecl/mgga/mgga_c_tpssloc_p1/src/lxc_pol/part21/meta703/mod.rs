//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2533;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta703<F: Float>(t13542: F, t13779: F, t2986: F, t13546: F, t13555: F, t13784: F, t13528: F, t1592: F, t42891: F, t973: F, t13812: F, t13822: F, t13881: F, t13886: F, t10263: F, t4506: F, t3082: F, t4622: F, t1040: F, t13941: F, t10231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48384, t48387, t48390, t48394, t48397, t48402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2533::<F>(t13542, t13779, t2986, t13546, t13555, t13784, t13528, t1592, t42891, t973, t13812, t13822);
        let (t48407, t48417, t48421, t48430, t48432, t48441) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2534::<F>(t13822, t13881, t973, t13886, t10263, t4506, t3082, t4622, t1040, t13941, t10231, t13555);
    (t48384, t48387, t48390, t48394, t48397, t48402, t48407, t48417, t48421, t48430, t48432, t48441)
}
