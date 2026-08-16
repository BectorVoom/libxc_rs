//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta718(t100674: f64, t100716: f64, t100763: f64, t100803: f64, t24987: f64, t7754: f64, t1983: f64, t2019: f64, t57806: f64, t25971: f64, t91655: f64, t26161: f64, t26162: f64, t75210: f64, t25994: f64, t7458: f64, t28817: f64, t6876: f64, t28826: f64, t83859: f64, t26149: f64, t7685: f64, t16524: f64, t26545: f64, t1873: f64, t66958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100805, t100828, t100833, t100835, t100838) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283(t100674, t100716, t100763, t100803, t24987, t7754, t1983, t2019, t57806, t25971, t91655, t26161, t26162, t75210);
        let (t100840, t100854, t100861, t100863, t100871, t100873) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284(t25994, t7458, t28817, t6876, t1983, t28826, t83859, t26149, t7685, t16524, t26545, t1873, t66958);
    (t100805, t100828, t100833, t100835, t100838, t100840, t100854, t100861, t100863, t100871, t100873)
}
