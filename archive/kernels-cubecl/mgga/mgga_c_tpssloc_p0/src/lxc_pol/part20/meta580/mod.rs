//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2146;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta580<F: Float>(t1022: F, t3120: F, t2250: F, t360: F, t1036: F, t10367: F, t1032: F, t10375: F, t370: F, t374: F, t376: F, t9697: F, t10908: F, t3109: F, t10446: F, t10997: F, t135: F, t973: F, t10480: F, t10483: F, t248: F, t3101: F, t10876: F, t10877: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43235, t43241, t43246, t43248, t43253) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2146::<F>(t1022, t3120, t2250, t360, t1036, t10367, t1032, t10375, t370, t374, t376, t9697);
        let (t43254, t43262, t43273, t43277, t43281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2147::<F>(t10908, t3109, t1036, t10446, t10997, t135, t973, t10480, t10483, t248, t3101, t10876, t10877);
    (t43235, t43241, t43246, t43248, t43253, t43254, t43262, t43273, t43277, t43281)
}
