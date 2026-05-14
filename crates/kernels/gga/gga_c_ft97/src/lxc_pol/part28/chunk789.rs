//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 789/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk789<F: Float>(t1389: F, t6616: F, t28: F, t6578: F, t7150: F, t1360: F, t925: F, t356: F, t461: F, t6681: F, t6615: F, t72: F, t1349: F, t1526: F, t1527: F, t2: F, t32663: F, t32675: F, t342: F, t343: F, t6673: F, t6678: F, t7298: F, t7299: F) -> (F, F, F, F, F, F, F, F) {
    let t34979 = t6616 * t1389;
    let t34980 = t28 * t34979;
    let t34985 = t6578 * t7150;
    let t34988 = t1360 * t925;
    let t34989 = t356 * t34988;
    let t34994 = t461 * t6681;
    let t35000 = t72 * t6615;
    let t35005 = (-t34985 * t7299 / 6.0 + t32663 + t1349 * t34989 / 18.0 + t1349 * t6678 / 3.0 - t7298 * t34994 / 6.0 - t32675 - t1526 * t1527 * t6673 / 12.0 - t342 * t343 * t35000 / 4.0) * t2;
    (t34979, t34980, t34985, t34988, t34989, t34994, t35000, t35005)
}
