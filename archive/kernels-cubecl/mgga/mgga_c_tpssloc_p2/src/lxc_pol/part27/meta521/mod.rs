//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta521<F: Float>(t343: F, t4540: F, t6734: F, t4571: F, t6765: F, t4630: F, t6755: F, t1611: F, t6758: F, t1036: F, t7586: F, t1409: F, t1933: F, t1937: F, t1618: F, t1622: F, t1935: F, t23433: F, t23443: F, t23447: F, t23449: F, t23463: F, t23469: F, t23529: F, t378: F, t6730: F, t7578: F) -> (F, F, F, F) {
        let (t25608, t25609, t25616, t25618, t25622, t25625, t25628) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1926::<F>(t343, t4540, t6734, t4571, t6765, t4630, t6755, t1611, t6758, t1036, t7586, t1409, t1933);
        let t25631 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1927::<F>(t1937, t25628, t1618, t1622, t1935, t23433, t23443, t23447, t23449, t23463, t23469, t23529, t25609, t25616, t25618, t25622, t25625, t378, t6730, t7578);
    (t25608, t25609, t25622, t25631)
}
