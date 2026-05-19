//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1379/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1379<F: Float>(t11982: F, t11986: F, t12044: F, t12060: F, t12065: F, t1265: F, t1445: F, t1457: F, t1572: F, t1596: F, t34262: F, t34263: F, t34266: F, t34270: F, t34276: F, t34279: F, t34282: F, t34285: F, t38299: F, t38399: F, t4673: F, t4753: F, t4842: F, t557: F, t574: F, t597: F) -> F {
    let t38512 = t34262 + t34263 + t34266 + t34270 + t34276 - t34279 - t34282 + t34285 + F::cast_from(0.35750489951850426669e0_f64) * t1596 * t12065 - F::cast_from(0.47667319935800568892e0_f64) * t12060 * t4753 - F::cast_from(0.14300195980740170668e1_f64) * t557 * t4673 * t12044 + F::cast_from(0.95334639871601137784e0_f64) * t1572 * t4673 * t11982 - F::cast_from(0.71500979903700853338e0_f64) * t4842 * t1457 * t38299 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t1445 * t11986 * t1265 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t38399;
    t38512
}
