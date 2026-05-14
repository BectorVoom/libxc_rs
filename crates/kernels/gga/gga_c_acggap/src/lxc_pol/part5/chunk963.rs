//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 963/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk963<F: Float>(t12313: F, t1629: F, t16560: F, t4159: F, t4180: F, t12726: F, t1642: F, t3077: F, t4189: F, t1160: F, t1603: F, t322: F, t407: F, t1410: F, t441: F, t1633: F, t17386: F) -> (F, F, F, F, F, F, F) {
    let t18938 = t12313 * t1629 * t16560;
    let t18941 = t4180 * t4159;
    let t18951 = t12726 * t1642;
    let t18953 = t3077 * t4189;
    let t18957 = t1160 * t1603 * t322 * t407;
    let t18973 = t441 * t1410;
    let t18977 = t17386 * t1633;
    (t18938, t18941, t18951, t18953, t18957, t18973, t18977)
}
