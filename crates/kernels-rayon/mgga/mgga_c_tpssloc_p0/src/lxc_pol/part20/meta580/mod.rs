//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2146;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta580(t1022: f64, t3120: f64, t2250: f64, t360: f64, t1036: f64, t10367: f64, t1032: f64, t10375: f64, t370: f64, t374: f64, t376: f64, t9697: f64, t10908: f64, t3109: f64, t10446: f64, t10997: f64, t135: f64, t973: f64, t10480: f64, t10483: f64, t248: f64, t3101: f64, t10876: f64, t10877: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43235, t43241, t43246, t43248, t43253) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2146(t1022, t3120, t2250, t360, t1036, t10367, t1032, t10375, t370, t374, t376, t9697);
        let (t43254, t43262, t43273, t43277, t43281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2147(t10908, t3109, t1036, t10446, t10997, t135, t973, t10480, t10483, t248, t3101, t10876, t10877);
    (t43235, t43241, t43246, t43248, t43253, t43254, t43262, t43273, t43277, t43281)
}
