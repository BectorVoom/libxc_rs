//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2016;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta591(t22705: f64, t22733: f64, t81228: f64, t22724: f64, t22927: f64, t22642: f64, t22643: f64, t6907: f64, t22644: f64, t81152: f64, t6891: f64, t81195: f64, t22649: f64, t6883: f64, t1372: f64, t212: f64, t6890: f64, t1988: f64, t81071: f64, t225: f64, t22942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81230, t81264, t81267, t81282, t81284) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2016(t22705, t22733, t81228, t22724, t22927, t22642, t22643, t6907, t22644, t81152, t6891, t81195);
        let (t81307, t81311, t81318, t81319, t81326) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2017(t22649, t6883, t1372, t212, t22642, t6890, t1988, t81071, t225, t22942, t22643);
    (t81230, t81264, t81267, t81282, t81284, t81307, t81311, t81318, t81319, t81326)
}
