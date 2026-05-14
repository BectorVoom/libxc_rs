//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 637/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk637<F: Float>(t13728: F, t2343: F, t2268: F, t11977: F, t888: F, t3691: F, t894: F, t11986: F, t2325: F, t883: F, t882: F, t12404: F, t12405: F, t12783: F, t12784: F, t12787: F, t12788: F, t12789: F, t12790: F, t12791: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13729 = t2343 * t13728;
    let t13730 = t2268 * t13729;
    let t13732 = t11977 * t888;
    let t13733 = t2268 * t13732;
    let t13735 = t894 * t3691;
    let t13736 = t2268 * t13735;
    let t13740 = t2325 * t883 * t11986;
    let t13741 = t882 * t13740;
    let t13749 = t12783 + t12784 / 2.0 + t12404 - t12405 - t12787 - t12788 + t12789 + t12790 + t12791;
    (t13729, t13730, t13732, t13733, t13735, t13736, t13740, t13741, t13749)
}
