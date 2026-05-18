//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1187/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1187<F: Float>(t33521: F, t34972: F, t1794: F, t3596: F, t1042: F, t1122: F, t1214: F, t124554: F, t124578: F, t124594: F, t124604: F, t124611: F, t124613: F, t124744: F, t124869: F, t1263: F, t131497: F, t131608: F, t1769: F, t2148: F, t247: F, t26969: F, t3153: F, t33398: F, t33461: F, t33462: F, t33469: F, t33478: F, t34901: F, t34960: F, t3719: F, t494: F, t5215: F, t5270: F, t5296: F, t5351: F, t5402: F, t5428: F, t5465: F, t7627: F, t8197: F, t96928: F) -> F {
    let t131799 = t34972 * t33521;
    let t131810 = t3596 * t1794;
    let t131815 = -F::new(0.28234466758480466999e-3) * t124611 * t124613 * t5351 * t96928 + F::new(0.24791552806034007214e-3) * t131608 * t5270 + F::new(0.3718732920905101082e-3) * t124578 * t1042 * t1263 * t1769 * t1122 - F::new(0.24791552806034007214e-3) * t124594 * t1042 * t5296 * t131497 + F::new(0.24791552806034007213e-3) * t124744 * t5402 + F::new(0.3427184259906141157e1) * t33461 * t33462 * t8197 * t7627 - F::new(0.52041769129231196772e1) * t2148 * t124604 * t26969 * t5428 + F::new(0.3718732920905101082e-3) * t124554 * t34901 - F::new(0.12395776403017003607e-3) * t131799 + F::new(0.56468933516960933998e-3) * t33398 * t247 * t3719 * t494 * t5215 + F::new(0.51407763898592117355e1) * t33469 * t33478 * t34960 * t1214 - F::new(0.17347256376410398924e1) * t124869 * t131810 * t3153 * t5465;
    t131815
}
