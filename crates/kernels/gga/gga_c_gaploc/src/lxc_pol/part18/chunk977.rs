//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 977/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk977<F: Float>(t119: F, t19223: F, t481: F, t19244: F, t1570: F, t21488: F, t565: F, t555: F, t189: F, t20369: F, t6508: F, t2310: F, t424: F, t1321: F, t880: F, t169: F, t18310: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23726 = t481 * t19223 * t119;
    let t23741 = t481 * t19244 * t119;
    let t23759 = t21488 * t565 * t1570;
    let t23763 = t21488 * t565 * t555;
    let t23767 = t21488 * t565 * t189;
    let t23911 = t6508 * t20369;
    let t23927 = t481 * t2310 * t424;
    let t23983 = t481 * t880 * t1321;
    let t24139 = t18310 * t169;
    (t23726, t23741, t23759, t23763, t23767, t23911, t23927, t23983, t24139)
}
