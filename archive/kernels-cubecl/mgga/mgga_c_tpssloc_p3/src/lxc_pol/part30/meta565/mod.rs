//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1931;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1932;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta565<F: Float>(t28354: F, t28430: F, t858: F, t218: F, t28406: F, t25224: F, t7488: F, t1880: F, t1492: F, t7510: F, t17090: F, t1912: F, t23231: F, t23252: F, t23262: F, t25206: F, t25209: F, t259: F, t26712: F, t26726: F, t28307: F, t28311: F, t28317: F, t4268: F, t5637: F, t5658: F, t6627: F, t7538: F, t855: F, t28304: F, t870: F, t25: F, t5664: F, t1408: F, t1530: F, t5660: F, t1877: F, t1915: F, t22959: F, t23295: F, t2522: F, t25358: F, t28242: F, t28249: F, t28252: F, t28256: F, t4314: F, t5397: F, t6670: F, t7475: F, t7541: F, t7545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28431, t28432, t28437, t28439, t28442, t28446) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1931::<F>(t28354, t28430, t858, t218, t28406, t25224, t7488, t1880, t1492, t7510, t17090, t1912, t23231, t23252, t23262, t25206, t25209, t259, t26712, t26726, t28307, t28311, t28317, t4268, t5637, t5658, t6627, t7538, t855);
        let (t28447, t28448) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1932::<F>(t28304, t28446, t870);
        let (t28456, t28459, t28462, t28469) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1933::<F>(t25, t5664, t1408, t1530, t5660, t1877, t1915, t22959, t23295, t2522, t25358, t28242, t28249, t28252, t28256, t28448, t4314, t5397, t6670, t7475, t7541, t7545);
    (t28431, t28432, t28437, t28439, t28442, t28447, t28448, t28456, t28459, t28462, t28469)
}
