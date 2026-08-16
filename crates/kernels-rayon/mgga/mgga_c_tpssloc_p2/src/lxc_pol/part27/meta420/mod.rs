//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta420(t1863: f64, t22550: f64, t6489: f64, t9231: f64, t1860: f64, t1865: f64, t22490: f64, t22493: f64, t22513: f64, t22516: f64, t22519: f64, t22523: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22544: f64, t22546: f64, t22549: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64, t5: f64, t112: f64, t1266: f64, t6534: f64, t652: f64, t192: f64, t532: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22551, t22554, t22557) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1727(t1863, t22550, t6489, t9231, t1860, t1865, t22490, t22493, t22513, t22516, t22519, t22523, t22527, t22531, t22534, t22537, t22544, t22546, t22549, t6486, t6490, t6492, t6495, t6506, t6510);
        let (t22558, t22559, t22561, t22563, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1728(t5, t22557, t112, t1266, t6534, t652, t192, t532, t1982);
    (t22551, t22554, t22558, t22559, t22561, t22563, t22573, t22574)
}
