//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta484<F: Float>(t1527: F, t6662: F, t2718: F, t225: F, t7492: F, t1484: F, t857: F, t865: F, t23270: F, t22986: F, t13065: F, t13463: F, t1528: F, t1912: F, t23206: F, t23209: F, t23231: F, t23232: F, t23278: F, t4268: F, t4273: F, t6627: F, t6632: F, t6663: F, t855: F, t866: F) -> (F, F, F, F, F, F) {
        let (t25184, t25188, t25191, t25192, t25193, t25196) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1790::<F>(t1527, t6662, t2718, t225, t7492, t1484, t857, t865, t23270, t22986, t13065, t13463, t1528, t1912, t23206, t23209, t23231, t23232, t23278, t4268, t4273, t6627, t6632, t6663, t855, t866);
    (t25184, t25188, t25191, t25192, t25193, t25196)
}
