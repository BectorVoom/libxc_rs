//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1082/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1082<F: Float>(t3268: F, t7143: F, t3057: F, t359: F, t42066: F, t3143: F, t36870: F, t1983: F, t1981: F, t42058: F, t11120: F, t3140: F, t1035: F, t11200: F, t1976: F, t27668: F, t995: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93920 = t7143 * t3268;
    let t93921 = t3057 * t93920;
    let t93968 = t42066 * t359;
    let t93982 = t36870 * t3143;
    let t93983 = t1983 * t93982;
    let t93994 = t1981 * t42058 * t7143;
    let t94014 = t3140 * t11120;
    let t94016 = t1983 * t94014 * t1035;
    let t94026 = t11200 * t1976;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94080 = t995 * t27668;
    (t93921, t93968, t93983, t93994, t94016, t94026, t94053, t94063, t94080)
}
