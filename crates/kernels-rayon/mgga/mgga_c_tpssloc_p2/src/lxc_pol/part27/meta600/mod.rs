//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2066;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2067;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta600(t23197: f64, t6547: f64, t23257: f64, t6562: f64, t794: f64, t23012: f64, t6568: f64, t225: f64, t23211: f64, t23205: f64, t82038: f64, t23242: f64, t81979: f64, t1914: f64, t40772: f64, t23547: f64, t381: f64, t23310: f64, t23384: f64, t23460: f64, t6686: f64, t23396: f64, t23326: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82230, t82236, t82259, t82287, t82294, t82296) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2066(t23197, t6547, t23257, t6562, t794, t23012, t6568, t225, t23211, t23205, t82038, t23242, t81979);
        let (t82312, t82357, t82380, t82382) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2067(t1914, t40772, t23547, t381, t23310, t23384, t23460, t6686);
        let (t82400, t82402) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2068(t23384, t23396, t23326, t6712);
    (t82230, t82236, t82259, t82287, t82294, t82296, t82312, t82357, t82380, t82382, t82400, t82402)
}
