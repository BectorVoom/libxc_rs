//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1256/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1256<F: Float>(t20671: F, t27007: F, t31047: F, t6907: F, t9263: F, t993: F, t2890: F, t9267: F, t10470: F, t4418: F, t10474: F, t4425: F, t20019: F, t26984: F, t6520: F, t10318: F, t4360: F, t4667: F) -> (F, F, F, F, F, F, F) {
    let t35119 = t31047 * t20671 * t27007;
    let t35120 = 0.42603251059911944084e0 * t35119;
    let t35122 = t9263 * t993 * t6907;
    let t35123 = 0.76685851907841499352e0 * t35122;
    let t35125 = t9267 * t2890 * t6907;
    let t35126 = 0.36425779656224712192e1 * t35125;
    let t35128 = 0.2556195063594716645e1 * t4418 * t10470;
    let t35130 = 0.1022478025437886658e1 * t4425 * t10474;
    let t35133 = 0.23833659967900284446e0 * t26984 * t20019 * t6520;
    let t35136 = 0.71500979903700853338e0 * t4360 * t10318 * t4667;
    (t35120, t35123, t35126, t35128, t35130, t35133, t35136)
}
