//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1856;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta575<F: Float>(t13231: F, t25084: F, t13353: F, t23146: F, t13225: F, t23069: F, t4159: F, t23062: F, t25106: F, t13176: F, t6613: F, t831: F, t25146: F, t2681: F, t23133: F, t4257: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F, t13228: F, t2628: F, t2678: F, t6605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87284, t87287, t87289, t87291, t87293, t87296) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1856::<F>(t13231, t25084, t13353, t23146, t13225, t23069, t4159, t23062, t25106, t13176, t6613, t831);
        let (t87298, t87300, t87304, t87306, t87308, t87312) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1857::<F>(t25146, t2681, t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835, t13228, t2628, t2678, t6605);
    (t87284, t87287, t87289, t87291, t87293, t87296, t87298, t87300, t87304, t87306, t87308, t87312)
}
