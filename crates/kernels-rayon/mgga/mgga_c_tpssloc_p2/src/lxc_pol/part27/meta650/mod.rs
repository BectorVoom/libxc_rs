//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2262;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta650(t12461: f64, t6995: f64, t26161: f64, t26163: f64, t22581: f64, t7685: f64, t24987: f64, t7000: f64, t25985: f64, t6876: f64, t6514: f64, t671: f64, t1868: f64, t2363: f64, t5107: f64, t652: f64, t6534: f64, t22574: f64, t56198: f64, t8643: f64, t26162: f64, t57802: f64, t22597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90034, t90036, t90038, t90040, t90041) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2262(t12461, t6995, t26161, t26163, t22581, t7685, t24987, t7000, t25985, t6876, t6514, t671);
        let (t90044, t90051, t90059, t90062, t90064) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2263(t1868, t2363, t5107, t652, t6534, t22574, t56198, t8643, t26162, t57802, t22597, t7685);
    (t90034, t90036, t90038, t90040, t90041, t90044, t90051, t90059, t90062, t90064)
}
