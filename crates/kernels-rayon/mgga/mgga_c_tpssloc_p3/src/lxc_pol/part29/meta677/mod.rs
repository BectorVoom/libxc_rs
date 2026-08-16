//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2268;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta677(t22561: f64, t7458: f64, t3652: f64, t652: f64, t7467: f64, t22579: f64, t7685: f64, t1874: f64, t55934: f64, t12725: f64, t6525: f64, t26168: f64, t6876: f64, t25989: f64, t83886: f64, t25994: f64, t4034: f64, t15857: f64, t1873: f64, t45632: f64, t26135: f64, t3941: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91759, t91762, t91763, t91765, t91767, t91769) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2268(t22561, t7458, t3652, t652, t7467, t22579, t7685, t1874, t55934, t12725, t6525, t26168, t6876);
        let (t91771, t91777, t91780, t91782, t91799) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269(t25989, t83886, t25994, t4034, t15857, t1873, t652, t1874, t45632, t26135, t3941, t671);
    (t91759, t91762, t91763, t91765, t91767, t91769, t91771, t91777, t91780, t91782, t91799)
}
