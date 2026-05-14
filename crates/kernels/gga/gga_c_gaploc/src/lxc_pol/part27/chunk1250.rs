//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1250/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1250<F: Float>(t1: F, t106: F, t12161: F, t316: F, t12206: F, t783: F, t12166: F, t12207: F, t12213: F, t1445: F, t1835: F, t1991: F, t1998: F, t2061: F, t2070: F, t28072: F, t28075: F, t28079: F, t28080: F, t28084: F, t28089: F, t32843: F, t32846: F, t32850: F, t32853: F, t590: F, t780: F) -> (F,) {
    let t38947 = t12161 * t1 * t106 * t316;
    let t38950 = t12206 * t783;
    let t38953 = -t32843 - t32846 - t28072 - t28075 - t28079 - 0.10224780254378866581e1 * t28080 + 0.20449560508757733162e1 * t28084 - t32850 - t28089 + 0.1022478025437886658e1 * t1991 * t12166 * t590 - t32853 - 0.23005755572352449806e1 * t1998 * t1445 * t12213 * t1835 + 0.71500979903700853338e0 * t2070 * t12207 + 0.35750489951850426669e0 * t2061 * t12207 + 0.71500979903700853338e0 * t780 * t38947 + 0.47667319935800568892e0 * t780 * t38950;
    (t38953,)
}
