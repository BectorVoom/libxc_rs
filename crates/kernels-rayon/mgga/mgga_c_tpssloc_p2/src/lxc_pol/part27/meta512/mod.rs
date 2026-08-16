//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta512(t2770: f64, t387: f64, t3961: f64, t23329: f64, t23581: f64, t7553: f64, t381: f64, t7577: f64, t6691: f64, t1052: f64, t14545: f64, t14552: f64, t1956: f64, t23327: f64, t25400: f64, t25403: f64, t25407: f64, t25410: f64, t25413: f64, t25416: f64, t25420: f64, t25425: f64, t25429: f64, t4660: f64, t4694: f64, t6687: f64, t6771: f64, t6776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25430, t25431, t25432, t25436, t25442, t25443, t25446) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1916(t2770, t387, t3961, t23329, t23581, t7553, t381, t7577, t6691, t1052, t14545, t14552, t1956, t23327, t25400, t25403, t25407, t25410, t25413, t25416, t25420, t25425, t25429, t4660, t4694, t6687, t6771, t6776);
    (t25430, t25431, t25432, t25436, t25442, t25443, t25446)
}
