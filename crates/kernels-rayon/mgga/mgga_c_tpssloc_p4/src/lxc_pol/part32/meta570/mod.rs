//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1943;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta570(t28354: f64, t28430: f64, t858: f64, t218: f64, t28406: f64, t25224: f64, t7488: f64, t1880: f64, t1492: f64, t7510: f64, t17090: f64, t1912: f64, t23231: f64, t23252: f64, t23262: f64, t25206: f64, t25209: f64, t259: f64, t26712: f64, t26726: f64, t28307: f64, t28311: f64, t28317: f64, t4268: f64, t5637: f64, t5658: f64, t6627: f64, t7538: f64, t855: f64, t28304: f64, t870: f64, t25: f64, t5664: f64, t1408: f64, t1530: f64, t5660: f64, t1877: f64, t1915: f64, t22959: f64, t23295: f64, t2522: f64, t25358: f64, t28242: f64, t28249: f64, t28252: f64, t28256: f64, t4314: f64, t5397: f64, t6670: f64, t7475: f64, t7541: f64, t7545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28431, t28432, t28437, t28439, t28442, t28446) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1942(t28354, t28430, t858, t218, t28406, t25224, t7488, t1880, t1492, t7510, t17090, t1912, t23231, t23252, t23262, t25206, t25209, t259, t26712, t26726, t28307, t28311, t28317, t4268, t5637, t5658, t6627, t7538, t855);
        let (t28447, t28448) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1943(t28304, t28446, t870);
        let (t28456, t28459, t28462, t28469) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1944(t25, t5664, t1408, t1530, t5660, t1877, t1915, t22959, t23295, t2522, t25358, t28242, t28249, t28252, t28256, t28448, t4314, t5397, t6670, t7475, t7541, t7545);
    (t28431, t28432, t28437, t28439, t28442, t28447, t28448, t28456, t28459, t28462, t28469)
}
