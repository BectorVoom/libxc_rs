//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1760;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1761;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta541(t22715: f64, t6887: f64, t6970: f64, t12225: f64, t22641: f64, t22690: f64, t6969: f64, t268: f64, t547: f64, t6559: f64, t22724: f64, t22927: f64, t22642: f64, t22643: f64, t6907: f64, t22644: f64, t81152: f64, t6891: f64, t1372: f64, t212: f64, t6890: f64, t1988: f64, t81071: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81186, t81187, t81195, t81197, t81228) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1760(t22715, t6887, t6970, t12225, t22641, t22690, t6969, t268, t547, t6559);
        let (t81264, t81267, t81281, t81284, t81311) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1761(t22724, t22927, t22642, t22643, t6907, t22644, t81152, t6891, t81195, t1372, t212, t6890);
        let (t81317, t81326) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1762(t1988, t81071, t225, t22643);
    (t81186, t81187, t81195, t81197, t81228, t81264, t81267, t81281, t81284, t81311, t81317, t81326)
}
