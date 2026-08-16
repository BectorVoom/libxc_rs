//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1064;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1065;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta171(t40: f64, t182: f64, t4095: f64, t145: f64, t4094: f64, t185: f64, t1472: f64, t751: f64, t1409: f64, t707: f64, t75: f64, t3966: f64, t607: f64, t767: f64, zeta_threshold: f64, t52: f64, t78: f64, t771: f64, t1489: f64, t2563: f64, t131: f64, t2570: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4097, t4098, t4099, t4100, t4101, t4103, t4104, t4110) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1064(t40, t182, t4095, t145, t4094, t185, t1472, t751, t1409, t707, t75, t3966, t607, t767, zeta_threshold);
        let (t4111, t4119) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1065(t52, t1409, t78, t3966, t607, t771, t4110, zeta_threshold);
        let (t4124, t4126, t4127) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1066(t1489, t2563, t131, t2570, t205);
    (t4097, t4098, t4099, t4100, t4101, t4103, t4104, t4111, t4119, t4124, t4126, t4127)
}
