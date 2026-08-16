//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1403/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1403(t1445: f64, t1457: f64, t31124: f64, t31126: f64, t31129: f64, t31131: f64, t31135: f64, t31144: f64, t31160: f64, t31163: f64, t31166: f64, t31169: f64, t31172: f64, t31175: f64, t34922: f64, t38388: f64, t38429: f64, t4540: f64, t567: f64) -> f64 {
    let t38787 = -t34922 + t31124 - 0.15337170381568299871e1_f64 * t31126 + 0.72851559312449424385e1_f64 * t31129 + 0.10224780254378866581e1_f64 * t31131 + t31135 + 0.30674340763136599742e1_f64 * t31144 + t31160 - t31163 - t31166 - t31169 - 0.21450293971110256001e1_f64 * t4540 * t1457 * t38429 + 0.23005755572352449806e1_f64 * t567 * t1445 * t38388 - t31172 - t31175;
    t38787
}
