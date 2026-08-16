//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1205;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1206;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1207;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1208;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1209;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta249(t1030: f64, t1940: f64, t354: f64, t1036: f64, t1942: f64, t1039: f64, t1000: f64, t1025: f64, t1046: f64, t1935: f64, t1937: f64, t350: f64, t378: f64, t6712: f64, t6716: f64, t6717: f64, t6723: f64, t6728: f64, t6730: f64, t6735: f64, t6742: f64, t6747: f64, t6750: f64, t6755: f64, t349: f64, t1946: f64, t225: f64, t1065: f64, t1955: f64, t3174: f64, t1949: f64, t968: f64, t1920: f64, t6688: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6758, t6759, t6763, t6764) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1205(t1030, t1940, t354, t1036, t1942, t1039);
        let t6765 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1206(t354, t6764);
        let t6768 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1207(t1000, t1025, t1046, t1935, t1937, t350, t378, t6712, t6716, t6717, t6723, t6728, t6730, t6735, t6742, t6747, t6750, t6755, t6759, t6763, t6765);
        let (t6769, t6771) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1208(t349, t6768, t1946, t225);
        let t6776 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1209(t1065, t1955, t3174);
        let (t6781, t6783, t6784) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1210(t1949, t968, t1920, t225, t6688);
    (t6758, t6759, t6763, t6764, t6765, t6768, t6769, t6771, t6776, t6781, t6783, t6784)
}
