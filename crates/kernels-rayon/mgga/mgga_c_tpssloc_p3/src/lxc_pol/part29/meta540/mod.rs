//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1931;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta540(t1307: f64, t26421: f64, t26446: f64, t26331: f64, t26403: f64, t5250: f64, t5287: f64, t6987: f64, t1338: f64, t7722: f64, t1352: f64, t16036: f64, t550: f64, t6976: f64, t1992: f64, t16040: f64, t1336: f64, t1814: f64, t22718: f64, t22726: f64, t22728: f64, t22730: f64, t22745: f64, t22752: f64, t22895: f64, t26434: f64, t26437: f64, t26442: f64, t3777: f64, t5234: f64, t5334: f64, t6988: f64, t6990: f64, t7745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26447, t26448, t26449, t26453, t26456, t26458, t26459, t26461) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1931(t1307, t26421, t26446, t26331, t26403, t5250, t5287, t6987, t1338, t7722, t1352, t16036, t550);
        let (t26462, t26466, t26467, t26470) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1932(t26461, t6976, t1992, t16040, t550, t1336, t1814, t22718, t22726, t22728, t22730, t22745, t22752, t22895, t26434, t26437, t26442, t26449, t26453, t26456, t26459, t3777, t5234, t5334, t6988, t6990, t7745);
    (t26447, t26448, t26453, t26456, t26458, t26459, t26461, t26462, t26466, t26467, t26470)
}
