//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta392(t1022: f64, t883: f64, t607: f64, t14211: f64, t3071: f64, t1615: f64, t360: f64, t4342: f64, t1025: f64, t10403: f64, t1041: f64, t10413: f64, t10909: f64, t10923: f64, t10927: f64, t14174: f64, t14180: f64, t14184: f64, t14189: f64, t14194: f64, t14198: f64, t14203: f64, t14207: f64, t2960: f64, t3070: f64, t3117: f64, t4590: f64, t4609: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14213, t14214, t14215, t14218, t14219, t14220, t14221, t14222, t14228) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1862(t1022, t883, t607, t14211, t3071, t1615, t360);
        let (t14229, t14230, t14233) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863(t14228, t4342, t3071, t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t14215, t14222, t2960, t3070, t3117, t4590, t4609, t973);
    (t14213, t14214, t14215, t14218, t14219, t14220, t14221, t14222, t14228, t14229, t14230, t14233)
}
