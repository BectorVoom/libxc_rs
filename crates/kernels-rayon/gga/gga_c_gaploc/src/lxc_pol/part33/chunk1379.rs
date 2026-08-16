//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1379/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1379(t11982: f64, t11986: f64, t12044: f64, t12060: f64, t12065: f64, t1265: f64, t1445: f64, t1457: f64, t1572: f64, t1596: f64, t34262: f64, t34263: f64, t34266: f64, t34270: f64, t34276: f64, t34279: f64, t34282: f64, t34285: f64, t38299: f64, t38399: f64, t4673: f64, t4753: f64, t4842: f64, t557: f64, t574: f64, t597: f64) -> f64 {
    let t38512 = t34262 + t34263 + t34266 + t34270 + t34276 - t34279 - t34282 + t34285 + 0.35750489951850426669e0_f64 * t1596 * t12065 - 0.47667319935800568892e0_f64 * t12060 * t4753 - 0.14300195980740170668e1_f64 * t557 * t4673 * t12044 + 0.95334639871601137784e0_f64 * t1572 * t4673 * t11982 - 0.71500979903700853338e0_f64 * t4842 * t1457 * t38299 - 0.46011511144704899612e1_f64 * t574 * t1445 * t11986 * t1265 + 0.11502877786176224903e2_f64 * t597 * t1445 * t38399;
    t38512
}
