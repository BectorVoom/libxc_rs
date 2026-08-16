//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta540<F: Float>(t1307: F, t26421: F, t26446: F, t26331: F, t26403: F, t5250: F, t5287: F, t6987: F, t1338: F, t7722: F, t1352: F, t16036: F, t550: F, t6976: F, t1992: F, t16040: F, t1336: F, t1814: F, t22718: F, t22726: F, t22728: F, t22730: F, t22745: F, t22752: F, t22895: F, t26434: F, t26437: F, t26442: F, t3777: F, t5234: F, t5334: F, t6988: F, t6990: F, t7745: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26447, t26448, t26449, t26453, t26456, t26458, t26459, t26461) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1887::<F>(t1307, t26421, t26446, t26331, t26403, t5250, t5287, t6987, t1338, t7722, t1352, t16036, t550);
        let (t26462, t26466, t26467, t26470) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1888::<F>(t26461, t6976, t1992, t16040, t550, t1336, t1814, t22718, t22726, t22728, t22730, t22745, t22752, t22895, t26434, t26437, t26442, t26449, t26453, t26456, t26459, t3777, t5234, t5334, t6988, t6990, t7745);
    (t26447, t26448, t26453, t26456, t26458, t26459, t26461, t26462, t26466, t26467, t26470)
}
