//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1417/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1417(t1: f64, t106: f64, t12161: f64, t316: f64, t12206: f64, t783: f64, t12166: f64, t12207: f64, t12213: f64, t1445: f64, t1835: f64, t1991: f64, t1998: f64, t2061: f64, t2070: f64, t28072: f64, t28075: f64, t28079: f64, t28080: f64, t28084: f64, t28089: f64, t32843: f64, t32846: f64, t32850: f64, t32853: f64, t590: f64, t780: f64) -> f64 {
    let t38947 = t12161 * t1 * t106 * t316;
    let t38950 = t12206 * t783;
    let t38953 = -t32843 - t32846 - t28072 - t28075 - t28079 - 0.10224780254378866581e1_f64 * t28080 + 0.20449560508757733162e1_f64 * t28084 - t32850 - t28089 + 0.1022478025437886658e1_f64 * t1991 * t12166 * t590 - t32853 - 0.23005755572352449806e1_f64 * t1998 * t1445 * t12213 * t1835 + 0.71500979903700853338e0_f64 * t2070 * t12207 + 0.35750489951850426669e0_f64 * t2061 * t12207 + 0.71500979903700853338e0_f64 * t780 * t38947 + 0.47667319935800568892e0_f64 * t780 * t38950;
    t38953
}
