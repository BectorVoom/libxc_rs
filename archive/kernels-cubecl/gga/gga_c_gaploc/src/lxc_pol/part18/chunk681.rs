//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 681/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk681<F: Float>(t6338: F, t885: F, t1349: F, t1358: F, t2268: F, t2336: F, t2346: F, t3818: F, t3822: F, t3833: F, t6286: F, t6290: F, t6293: F, t6297: F, t6300: F, t6302: F, t6305: F, t6309: F, t6313: F, t6317: F, t6322: F, t6325: F, t6328: F, t6334: F) -> (F, F) {
    let t6339 = t6338 * t885;
    let t6341 = F::cast_from(0.63233348079280332442e-2_f64) * t1349 * t6286 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t6290 + F::cast_from(0.23712505529730124666e-2_f64) * t6293 - F::cast_from(0.11856252764865062333e-2_f64) * t6297 + F::cast_from(0.11856252764865062333e-2_f64) * t6300 + F::cast_from(0.31616674039640166222e-2_f64) * t6302 + F::cast_from(0.1138200265427045984e0_f64) * t6305 * t2346 - F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t6309 + F::cast_from(0.15176003539027279786e0_f64) * t6313 * t2346 + F::cast_from(0.34146007962811379518e0_f64) * t2268 * t6317 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6322 - F::cast_from(0.19918504644973304719e0_f64) * t2268 * t6325 + F::cast_from(0.36886119712913527259e-2_f64) * t6328 + F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t2336 + F::cast_from(0.7588001769513639893e-1_f64) * t3818 * t2336 - F::cast_from(0.31616674039640166222e-2_f64) * t6334 + F::cast_from(0.11856252764865062333e-2_f64) * t6339;
    (t6339, t6341)
}
