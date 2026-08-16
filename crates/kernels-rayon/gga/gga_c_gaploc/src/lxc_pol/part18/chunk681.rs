//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 681/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk681(t6338: f64, t885: f64, t1349: f64, t1358: f64, t2268: f64, t2336: f64, t2346: f64, t3818: f64, t3822: f64, t3833: f64, t6286: f64, t6290: f64, t6293: f64, t6297: f64, t6300: f64, t6302: f64, t6305: f64, t6309: f64, t6313: f64, t6317: f64, t6322: f64, t6325: f64, t6328: f64, t6334: f64) -> (f64, f64) {
    let t6339 = t6338 * t885;
    let t6341 = 0.63233348079280332442e-2_f64 * t1349 * t6286 - 0.63233348079280332442e-2_f64 * t1358 * t6290 + 0.23712505529730124666e-2_f64 * t6293 - 0.11856252764865062333e-2_f64 * t6297 + 0.11856252764865062333e-2_f64 * t6300 + 0.31616674039640166222e-2_f64 * t6302 + 0.1138200265427045984e0_f64 * t6305 * t2346 - 0.56910013271352299198e-1_f64 * t3822 * t6309 + 0.15176003539027279786e0_f64 * t6313 * t2346 + 0.34146007962811379518e0_f64 * t2268 * t6317 - 0.17073003981405689759e0_f64 * t2268 * t6322 - 0.19918504644973304719e0_f64 * t2268 * t6325 + 0.36886119712913527259e-2_f64 * t6328 + 0.56910013271352299198e-1_f64 * t3833 * t2336 + 0.7588001769513639893e-1_f64 * t3818 * t2336 - 0.31616674039640166222e-2_f64 * t6334 + 0.11856252764865062333e-2_f64 * t6339;
    (t6339, t6341)
}
