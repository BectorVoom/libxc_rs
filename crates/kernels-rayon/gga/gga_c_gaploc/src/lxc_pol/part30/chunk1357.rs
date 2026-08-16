//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1357/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1357(t10525: f64, t10526: f64, t34246: f64, t8063: f64, t9285: f64, t2877: f64, t30642: f64, t10151: f64, t10152: f64, t10337: f64, t10341: f64, t10345: f64, t10451: f64, t1323: f64, t1445: f64, t1450: f64, t1562: f64, t1599: f64, t31534: f64, t34216: f64, t34220: f64, t34223: f64, t34242: f64, t34245: f64, t4614: f64, t4730: f64, t4753: f64, t4950: f64, t597: f64) -> f64 {
    let t34249 = 0.21450293971110256001e1_f64 * t10525 * t10526 * t34246;
    let t34251 = 0.47667319935800568892e0_f64 * t9285 * t8063;
    let t34253 = 0.71500979903700853338e0_f64 * t30642 * t2877;
    let t34254 = 0.11502877786176224903e2_f64 * t597 * t1445 * t31534 - 0.47667319935800568892e0_f64 * t10337 * t4753 - t34216 - t34220 + 0.14300195980740170668e1_f64 * t4950 * t10345 - 0.21450293971110256002e1_f64 * t1599 * t34223 + 0.30674340763136599741e2_f64 * t597 * t4614 * t10152 - 0.18404604457881959845e2_f64 * t1562 * t4614 * t10341 - 0.61348681526273199482e1_f64 * t1450 * t4614 * t10451 + 0.46011511144704899612e1_f64 * t4730 * t1445 * t10151 * t1323 - t34242 + t34245 - t34249 + t34251 + t34253;
    t34254
}
