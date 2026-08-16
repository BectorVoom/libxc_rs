//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1112/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1112(t31084: f64, t33159: f64, t1983: f64, t16524: f64, t8319: f64, t1458: f64, t576: f64, t1873: f64, t7467: f64, t3941: f64, t5371: f64, t8326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33160 = t31084 * t33159;
    let t33162 = 3.0_f64 * t1983 * t33160;
    let t33184 = 27.0_f64 * t16524 * t8319;
    let t33185 = t576 * t1458;
    let t33187 = 27.0_f64 * t33185 * t8319;
    let t33188 = t1873 * t7467;
    let t33190 = 54.0_f64 * t3941 * t33188;
    let t33191 = t5371 * t8326;
    (t33160, t33162, t33184, t33185, t33187, t33188, t33190, t33191)
}
