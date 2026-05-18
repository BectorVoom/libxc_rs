//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 687/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk687<F: Float>(t447: F, t6417: F, t1064: F, t105: F, t1063: F, t1079: F, t1083: F, t1212: F, t2264: F, t2268: F, t2269: F, t2296: F, t2350: F, t380: F, t3818: F, t3833: F, t419: F, t6305: F, t6313: F, t6342: F, t6345: F, t6348: F, t6353: F, t6356: F, t6396: F, t877: F, t889: F) -> (F, F) {
    let t6418 = t6417 * t447;
    let t6419 = t1064 * t6418;
    let t6422 = -F::new(0.85365019907028448797e-1) * t2268 * t6342 + F::new(0.28455006635676149599e-1) * t2268 * t6345 + F::new(0.56910013271352299198e-1) * t2268 * t6348 + F::new(0.56910013271352299198e-1) * t6305 * t2269 - F::new(0.56910013271352299198e-1) * t1063 * t6353 - F::new(0.28455006635676149599e-1) * t1063 * t6356 - F::new(0.7588001769513639893e-1) * t3818 * t2264 + F::new(0.28455006635676149599e-1) * t105 * t6396 + F::new(0.12646669615856066488e-1) * t1079 * t877 - F::new(0.12646669615856066488e-1) * t1079 * t889 + F::new(0.7588001769513639893e-1) * t1083 * t877 + F::new(0.7588001769513639893e-1) * t380 * t2296 + F::new(0.28455006635676149599e-1) * t1212 * t877 + F::new(0.56910013271352299198e-1) * t419 * t2296 - F::new(0.56910013271352299198e-1) * t419 * t2350 + F::new(0.7588001769513639893e-1) * t6313 * t2269 - F::new(0.56910013271352299198e-1) * t3833 * t2264 + F::new(0.56910013271352299198e-1) * t1063 * t6419;
    (t6418, t6422)
}
