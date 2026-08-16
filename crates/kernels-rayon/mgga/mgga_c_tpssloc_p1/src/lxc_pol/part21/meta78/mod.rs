//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk563;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk564;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk565;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk566;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk567;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk568;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta78(t1603: f64, t381: f64, t1409: f64, t998: f64, t974: f64, t225: f64, t68: f64, t369: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64, t360: f64, t1021: f64, t248: f64, t1044: f64, t1539: f64, t1020: f64, t1038: f64, t1041: f64, t378: f64, t973: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1604, t1606, t1607, t1610) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk563(t1603, t381, t1409, t998, t974, t225);
        let t1611 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk564(t1610, t68);
        let (t1612, t1615) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk565(t1611, t369, t1545, t1559, t1585, t1587, t1591);
        let t1616 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk566(t1615, t360);
        let t1618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk567(t1021, t1616, t248);
        let t1622 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk568(t1044, t1539, t248);
        let t1625 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk569(t1020, t1038, t1041, t1607, t1612, t1618, t1622, t378, t973, t997);
    (t1604, t1606, t1607, t1610, t1611, t1612, t1615, t1616, t1618, t1622, t1625)
}
