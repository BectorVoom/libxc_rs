//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1174/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1174<F: Float>(t245: F, t107799: F, t107829: F, t107869: F, t107901: F, t107949: F, t107982: F, t109515: F, t109561: F, t109601: F, t109636: F, t109667: F, t109704: F, t109749: F, t109777: F, t109811: F, t111543: F, t1459: F, t1577: F, t1580: F, t18: F, t21: F, t24857: F, t28474: F, t28484: F, t363: F, t5: F, t6200: F, t6953: F, t7742: F, t920: F) -> (F,) {
    let t246 = 10000000.0 <= t245;
    let t111570 = piecewise3(t246, 0.0, t5 * (t107799 + t107829 + t107869 + t107901 + t107949 + t107982 + t109515 + t109561 + t109601 + t109636 + t109667 + t109704 + t109749 + t109777 + t109811 + t111543) * t21 / 4.0 + t5 * t28474 * t363 / 2.0 + t5 * t6953 * t1580 / 4.0 + t5 * t24857 * t920 / 4.0 + t5 * t6200 * t18 * t1577 + t5 * t1459 * t1577 / 2.0 - 3.0 / 2.0 * t5 * t28484 * t7742);
    (t111570,)
}
