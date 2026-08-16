//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1023/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1023<F: Float>(t12259: F, t1445: F, t1457: F, t10022: F, t10026: F, t10030: F, t10042: F, t11059: F, t11063: F, t11067: F, t11071: F, t11108: F, t11111: F, t11118: F, t11121: F, t12252: F, t12256: F, t2004: F, t2028: F, t2639: F, t807: F) -> (F, F, F) {
    let t12260 = t1445 * t12259;
    let t12263 = t1457 * t12259;
    let t12267 = -t11059 + t11063 - t11067 + t11071 + t11108 - t10022 - F::cast_from(0.39722766613167140743e-1_f64) * t12252 * t2028 - F::cast_from(0.10725146985555128001e1_f64) * t12256 * t2639 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t12260 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t12263 - t10026 - F::cast_from(0.51123901271894332903e0_f64) * t10030 + t10042 - t11111 - t11118 + t11121;
    (t12260, t12263, t12267)
}
