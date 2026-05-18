//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1359/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1359<F: Float>(t10525: F, t10526: F, t34246: F, t8063: F, t9285: F, t2877: F, t30642: F, t10151: F, t10152: F, t10337: F, t10341: F, t10345: F, t10451: F, t1323: F, t1445: F, t1450: F, t1562: F, t1599: F, t31534: F, t34216: F, t34220: F, t34223: F, t34242: F, t34245: F, t4614: F, t4730: F, t4753: F, t4950: F, t597: F) -> F {
    let t34249 = F::new(0.21450293971110256001e1) * t10525 * t10526 * t34246;
    let t34251 = F::new(0.47667319935800568892e0) * t9285 * t8063;
    let t34253 = F::new(0.71500979903700853338e0) * t30642 * t2877;
    let t34254 = F::new(0.11502877786176224903e2) * t597 * t1445 * t31534 - F::new(0.47667319935800568892e0) * t10337 * t4753 - t34216 - t34220 + F::new(0.14300195980740170668e1) * t4950 * t10345 - F::new(0.21450293971110256002e1) * t1599 * t34223 + F::new(0.30674340763136599741e2) * t597 * t4614 * t10152 - F::new(0.18404604457881959845e2) * t1562 * t4614 * t10341 - F::new(0.61348681526273199482e1) * t1450 * t4614 * t10451 + F::new(0.46011511144704899612e1) * t4730 * t1445 * t10151 * t1323 - t34242 + t34245 - t34249 + t34251 + t34253;
    t34254
}
