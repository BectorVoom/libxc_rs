//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 858/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk858<F: Float>(t43832: F, t43007: F, t739: F, t1991: F, t590: F, t43107: F, t1890: F, t1966: F, t10948: F, t11016: F, t13012: F, t2087: F, t4614: F, t1445: F, t1998: F, t43306: F, t43800: F, t43803: F, t43806: F, t43809: F, t43812: F, t43815: F, t43817: F, t43820: F, t43822: F, t43825: F, t43830: F, t701: F) -> (F,) {
    let t43833 = 0.11502877786176224903e1 * t43832;
    let t43834 = t739 * t43007;
    let t43836 = t1991 * t43834 * t590;
    let t43838 = t739 * t43107;
    let t43841 = 0.1022478025437886658e1 * t1991 * t43838 * t590;
    let t43842 = t1890 * t43007;
    let t43844 = t1966 * t43842 * t590;
    let t43849 = 0.25561950635947166451e1 * t1966 * t1890 * t43107 * t590;
    let t43854 = t10948 * t11016;
    let t43858 = 0.92023022289409799224e1 * t2087 * t4614 * t13012;
    let t43859 = t43800 - t43803 + t43806 - t43809 - 0.29792074959875355558e-1 * t43812 + 0.92023022289409799224e1 * t43815 - 0.29792074959875355558e-1 * t43817 + t43820 + t43822 - 0.71500979903700853338e0 * t43825 - t43830 + t43833 + 0.20449560508757733161e1 * t43836 + t43841 - 0.51123901271894332902e1 * t43844 - t43849 - 0.23005755572352449806e1 * t1998 * t1445 * t43306 * t701 - 0.14300195980740170668e1 * t43854 - t43858;
    (t43859,)
}
