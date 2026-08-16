//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta528(t27363: f64, t67: f64, t1864: f64, t1860: f64, t2110: f64, t24520: f64, t24526: f64, t26055: f64, t26063: f64, t26067: f64, t26090: f64, t27332: f64, t27341: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t7256: f64, t7259: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t5: f64, t25: f64, t265: f64, t394: f64, t27326: f64, t112: f64, t25882: f64, t1409: f64, t2116: f64, t25398: f64, t3966: f64, t40: f64, t607: f64, t7274: f64, t7992: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t27364, t27365, t27368) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1863(t27363, t67, t1864, t1860, t2110, t24520, t24526, t26055, t26063, t26067, t26090, t27332, t27341, t6486, t6492, t6495, t7246, t7256, t7259, t7432, t7435, t7975, t7978);
        let (t27370, t27371, t27373, t27380) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1864(t5, t25, t265, t394, t27326, t27368, t112, t25882, t1409, t2116, t25398, t3966, t40, t607, t7274, t7992, dens_threshold, rho0, zeta_threshold);
    (t27364, t27365, t27370, t27371, t27373, t27380)
}
