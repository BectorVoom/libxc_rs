//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta365<F: Float>(t248: F, t3521: F, t4733: F, t1227: F, t3536: F, t4997: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F) -> (F, F, F, F, F, F, F) {
        let (t15486, t15488, t15490, t15492, t15494, t15495, t15498) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1417::<F>(t248, t3521, t4733, t1227, t3536, t4997, t3570, t5012, t1213, t3535, t5018, t1202, t5023);
    (t15486, t15488, t15490, t15492, t15494, t15495, t15498)
}
