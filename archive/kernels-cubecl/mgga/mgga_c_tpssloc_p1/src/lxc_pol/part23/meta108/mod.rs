//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta108<F: Float>(t1409: F, t751: F, t707: F, t75: F, t78: F, t1489: F, t2563: F, t131: F, t2570: F, t205: F, t1484: F, t213: F) -> (F, F, F, F, F, F, F, F) {
        let (t4101, t4102, t4104, t4111, t4124, t4126, t4127, t4128) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk581::<F>(t1409, t751, t707, t75, t78, t1489, t2563, t131, t2570, t205, t1484, t213);
    (t4101, t4102, t4104, t4111, t4124, t4126, t4127, t4128)
}
