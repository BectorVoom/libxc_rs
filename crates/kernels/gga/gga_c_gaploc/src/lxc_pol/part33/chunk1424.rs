//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1424/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1424<F: Float>(t12218: F, t1865: F, t12206: F, t2097: F, t12224: F, t12259: F, t12260: F, t1445: F, t1457: F, t2004: F, t2087: F, t2096: F, t2178: F, t28529: F, t33205: F, t33210: F, t33212: F, t33215: F, t33218: F, t33221: F, t33223: F, t33225: F, t38970: F, t4614: F, t4673: F, t5983: F, t813: F) -> (F, F) {
    let t39058 = t12218 * t1865;
    let t39073 = t12206 * t2097;
    let t39076 = -F::cast_from(0.62115540045351614476e2_f64) * t2087 * t1445 * t39058 - t33205 - F::cast_from(0.12269736305254639896e2_f64) * t813 * t4614 * t12224 - t33210 - t33212 + t28529 + t33215 - t33218 + t33221 - t33223 - t33225 + F::cast_from(0.46011511144704899612e1_f64) * t2178 * t12260 - F::cast_from(0.71500979903700853338e0_f64) * t5983 * t1457 * t38970 + F::cast_from(0.47667319935800568892e0_f64) * t2004 * t4673 * t12259 - F::cast_from(0.25025342966295298669e1_f64) * t2096 * t39073;
    (t39058, t39076)
}
