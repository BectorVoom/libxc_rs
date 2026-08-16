//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta528<F: Float>(t27363: F, t67: F, t1864: F, t1860: F, t2110: F, t24520: F, t24526: F, t26055: F, t26063: F, t26067: F, t26090: F, t27332: F, t27341: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F, t7432: F, t7435: F, t7975: F, t7978: F, t5: F, t25: F, t265: F, t394: F, t27326: F, t112: F, t25882: F, t1409: F, t2116: F, t25398: F, t3966: F, t40: F, t607: F, t7274: F, t7992: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
        let (t27364, t27365, t27368) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1863::<F>(t27363, t67, t1864, t1860, t2110, t24520, t24526, t26055, t26063, t26067, t26090, t27332, t27341, t6486, t6492, t6495, t7246, t7256, t7259, t7432, t7435, t7975, t7978);
        let (t27370, t27371, t27373, t27380) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1864::<F>(t5, t25, t265, t394, t27326, t27368, t112, t25882, t1409, t2116, t25398, t3966, t40, t607, t7274, t7992, dens_threshold, rho0, zeta_threshold);
    (t27364, t27365, t27370, t27371, t27373, t27380)
}
