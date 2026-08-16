//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1161/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1161<F: Float>(t31492: F, t2321: F, t26629: F, t9074: F, t10141: F, t6313: F, t10215: F, t203: F, t447: F, t10122: F, t1305: F, t10124: F, t10127: F, t10157: F, t1063: F, t1064: F, t1529: F, t1595: F, t2268: F, t29850: F, t29852: F, t31488: F, t31490: F, t3340: F, t3358: F, t3833: F, t6305: F) -> (F, F, F, F) {
    let t31493 = F::cast_from(0.11856252764865062333e-2_f64) * t31492;
    let t31495 = t9074 * t26629 * t2321;
    let t31496 = F::cast_from(0.23712505529730124666e-2_f64) * t31495;
    let t31498 = F::cast_from(0.15176003539027279786e0_f64) * t6313 * t10141;
    let t31501 = t203 * t10215;
    let t31502 = t31501 * t447;
    let t31509 = t10122 * t1305;
    let t31520 = -t31488 + t31490 + t31493 + t31496 + t29850 - t29852 + t31498 + F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t10124 + F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t1064 * t31502 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t1595 * t3340 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t1064 * t31509 + F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t10127 - F::cast_from(0.19918504644973304719e0_f64) * t2268 * t1529 * t3358 - F::cast_from(0.2276400530854091968e0_f64) * t6313 * t10157;
    (t31501, t31502, t31509, t31520)
}
