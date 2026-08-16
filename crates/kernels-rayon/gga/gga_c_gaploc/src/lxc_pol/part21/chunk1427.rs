//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1427/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1427(t12218: f64, t1865: f64, t12206: f64, t2097: f64, t12224: f64, t12259: f64, t12260: f64, t1445: f64, t1457: f64, t2004: f64, t2087: f64, t2096: f64, t2178: f64, t28529: f64, t33205: f64, t33210: f64, t33212: f64, t33215: f64, t33218: f64, t33221: f64, t33223: f64, t33225: f64, t38970: f64, t4614: f64, t4673: f64, t5983: f64, t813: f64) -> (f64, f64) {
    let t39058 = t12218 * t1865;
    let t39073 = t12206 * t2097;
    let t39076 = -0.62115540045351614476e2_f64 * t2087 * t1445 * t39058 - t33205 - 0.12269736305254639896e2_f64 * t813 * t4614 * t12224 - t33210 - t33212 + t28529 + t33215 - t33218 + t33221 - t33223 - t33225 + 0.46011511144704899612e1_f64 * t2178 * t12260 - 0.71500979903700853338e0_f64 * t5983 * t1457 * t38970 + 0.47667319935800568892e0_f64 * t2004 * t4673 * t12259 - 0.25025342966295298669e1_f64 * t2096 * t39073;
    (t39058, t39076)
}
