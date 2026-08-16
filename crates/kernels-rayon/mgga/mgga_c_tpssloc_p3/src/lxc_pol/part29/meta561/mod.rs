//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1967;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta561(t27480: f64, t27529: f64, t27568: f64, t27739: f64, t1241: f64, t2154: f64, t5088: f64, t3598: f64, t1751: f64, t7299: f64, t7302: f64, t24574: f64, t8015: f64, t1238: f64, t14980: f64, t1761: f64, t2155: f64, t24589: f64, t24880: f64, t27406: f64, t27422: f64, t27424: f64, t27427: f64, t27434: f64, t27438: f64, t27441: f64, t27446: f64, t3487: f64, t498: f64, t7283: f64, t7288: f64, t8061: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t27741, t27742, t27747, t27751, t27752, t27755) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1967(t27480, t27529, t27568, t27739, t1241, t2154, t5088, t3598, t1751, t7299, t7302, t24574, t8015);
        let t27757 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1968(t1238, t14980, t1761, t2155, t24589, t24880, t27406, t27422, t27424, t27427, t27434, t27438, t27441, t27446, t27742, t27747, t27752, t27755, t3487, t498, t7283, t7288, t8061);
    (t27741, t27742, t27747, t27751, t27752, t27757)
}
