//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta579(t10868: f64, t820: f64, t3070: f64, t3072: f64, t10489: f64, t3117: f64, t1015: f64, t10472: f64, t42559: f64, t10870: f64, t3048: f64, t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64, t10510: f64, t3109: f64, t10965: f64, t3053: f64, t3082: f64, t3094: f64, t10895: f64, t10952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43198, t43200, t43206, t43211, t43214, t43216) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144(t10868, t820, t3070, t3072, t10489, t3117, t1015, t10472, t42559, t10870, t3048, t204, t376);
        let (t43219, t43221, t43226, t43228, t43233) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2145(t1020, t1023, t248, t43216, t10510, t3109, t10965, t3053, t3082, t3094, t10895, t10952);
    (t43198, t43200, t43206, t43211, t43214, t43216, t43219, t43221, t43226, t43228, t43233)
}
