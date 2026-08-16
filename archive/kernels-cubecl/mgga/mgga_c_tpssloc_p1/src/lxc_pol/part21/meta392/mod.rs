//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta392<F: Float>(t1022: F, t883: F, t607: F, t14211: F, t3071: F, t1615: F, t360: F, t4342: F, t1025: F, t10403: F, t1041: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t14198: F, t14203: F, t14207: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14213, t14214, t14215, t14218, t14219, t14220, t14221, t14222, t14228) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1862::<F>(t1022, t883, t607, t14211, t3071, t1615, t360);
        let (t14229, t14230, t14233) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863::<F>(t14228, t4342, t3071, t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t14215, t14222, t2960, t3070, t3117, t4590, t4609, t973);
    (t14213, t14214, t14215, t14218, t14219, t14220, t14221, t14222, t14228, t14229, t14230, t14233)
}
