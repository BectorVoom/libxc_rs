//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 672/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk672<F: Float>(t2299: F, t2754: F, t3394: F, t4130: F, t10241: F, t9448: F, t9439: F, t1339: F, t31585: F, t20550: F, t2875: F, t544: F, t6514: F, t1359: F, t2925: F, t299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34604 = t2299 * t2754;
    let t34688 = t4130 * t3394;
    let t34814 = t9448 * t10241;
    let t34818 = t9439 * t10241;
    let t34890 = t1339 * t3394;
    let t35045 = t1339 * t31585;
    let t35091 = t4130 * t2754;
    let t35101 = t20550 * t10241;
    let t35180 = t544 * t6514 * t2875;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t35385 = t299 * t2925;
    (t34604, t34688, t34814, t34818, t34890, t35045, t35091, t35101, t35180, t35215, t35216, t35385)
}
