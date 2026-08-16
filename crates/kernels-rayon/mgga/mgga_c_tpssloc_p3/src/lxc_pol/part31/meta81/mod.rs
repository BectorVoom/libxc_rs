//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk506;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk507;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk508;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk509;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta81(t1610: f64, t68: f64, t369: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64, t360: f64, t1021: f64, t248: f64, t1044: f64, t1539: f64, t1020: f64, t1038: f64, t1041: f64, t1607: f64, t378: f64, t973: f64, t997: f64, t349: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1611, t1612, t1615) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk506(t1610, t68, t369, t1545, t1559, t1585, t1587, t1591);
        let t1616 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk507(t1615, t360);
        let (t1618, t1622) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk508(t1021, t1616, t248, t1044, t1539);
        let t1625 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk509(t1020, t1038, t1041, t1607, t1612, t1618, t1622, t378, t973, t997);
        let (t1626, t1629) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk510(t1625, t349, t1615, t381);
    (t1611, t1612, t1615, t1616, t1618, t1622, t1625, t1626, t1629)
}
