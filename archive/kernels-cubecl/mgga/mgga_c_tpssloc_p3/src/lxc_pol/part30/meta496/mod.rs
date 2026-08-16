//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta496<F: Float>(t2770: F, t387: F, t3961: F, t23329: F, t23581: F, t7553: F, t381: F, t7577: F, t6691: F, t1052: F, t14545: F, t14552: F, t1956: F, t23327: F, t25400: F, t25403: F, t25407: F, t25410: F, t25413: F, t25416: F, t25420: F, t25425: F, t25429: F, t4660: F, t4694: F, t6687: F, t6771: F, t6776: F) -> (F, F, F, F, F, F, F) {
        let (t25430, t25431, t25432, t25436, t25442, t25443, t25446) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1811::<F>(t2770, t387, t3961, t23329, t23581, t7553, t381, t7577, t6691, t1052, t14545, t14552, t1956, t23327, t25400, t25403, t25407, t25410, t25413, t25416, t25420, t25425, t25429, t4660, t4694, t6687, t6771, t6776);
    (t25430, t25431, t25432, t25436, t25442, t25443, t25446)
}
