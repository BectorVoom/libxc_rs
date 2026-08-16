//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1161/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1161(t31492: f64, t2321: f64, t26629: f64, t9074: f64, t10141: f64, t6313: f64, t10215: f64, t203: f64, t447: f64, t10122: f64, t1305: f64, t10124: f64, t10127: f64, t10157: f64, t1063: f64, t1064: f64, t1529: f64, t1595: f64, t2268: f64, t29850: f64, t29852: f64, t31488: f64, t31490: f64, t3340: f64, t3358: f64, t3833: f64, t6305: f64) -> (f64, f64, f64, f64) {
    let t31493 = 0.11856252764865062333e-2_f64 * t31492;
    let t31495 = t9074 * t26629 * t2321;
    let t31496 = 0.23712505529730124666e-2_f64 * t31495;
    let t31498 = 0.15176003539027279786e0_f64 * t6313 * t10141;
    let t31501 = t203 * t10215;
    let t31502 = t31501 * t447;
    let t31509 = t10122 * t1305;
    let t31520 = -t31488 + t31490 + t31493 + t31496 + t29850 - t29852 + t31498 + 0.56910013271352299198e-1_f64 * t3833 * t10124 + 0.56910013271352299198e-1_f64 * t1063 * t1064 * t31502 + 0.28455006635676149599e-1_f64 * t2268 * t1595 * t3340 + 0.28455006635676149599e-1_f64 * t1063 * t1064 * t31509 + 0.56910013271352299198e-1_f64 * t6305 * t10127 - 0.19918504644973304719e0_f64 * t2268 * t1529 * t3358 - 0.2276400530854091968e0_f64 * t6313 * t10157;
    (t31501, t31502, t31509, t31520)
}
