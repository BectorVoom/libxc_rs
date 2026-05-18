//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 759/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk759<F: Float>(t1339: F, t3394: F, t31585: F, t2754: F, t4130: F, t10241: F, t20550: F, t1: F, t31740: F, t544: F, t2875: F, t6514: F) -> (F, F, F, F, F, F) {
    let t34890 = t1339 * t3394;
    let t35045 = t1339 * t31585;
    let t35091 = t4130 * t2754;
    let t35101 = t20550 * t10241;
    let t35106 = t544 * t31740 * t1;
    let t35180 = t544 * t6514 * t2875;
    (t34890, t35045, t35091, t35101, t35106, t35180)
}
