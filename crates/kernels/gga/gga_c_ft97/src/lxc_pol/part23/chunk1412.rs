//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1412/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1412<F: Float>(t25462: F, t31352: F, t10683: F, t112520: F, t125852: F, t125898: F, t125950: F, t125984: F, t126028: F, t126069: F, t126111: F, t126156: F, t126197: F, t126245: F, t126286: F, t126328: F, t126372: F, t126417: F, t126471: F, t126518: F, t126562: F, t126593: F, t126645: F, t126685: F, t127728: F, t127919: F, t127946: F, t127984: F, t128032: F, t128066: F, t128101: F, t128145: F, t128370: F, t128413: F, t128455: F, t128495: F, t128538: F, t128567: F, t1466: F, t193: F, t19399: F, t19409: F, t25459: F, t25465: F, t2665: F, t28997: F, t29002: F, t29008: F, t301: F, t3051: F, t31340: F, t31674: F, t317: F, t44280: F, t4969: F, t6210: F, t6216: F, t6217: F, t6962: F, t798: F, t98416: F) -> (F,) {
    let t128589 = t25462 * t31352;
    let t128599 = -t29008 * t28997 / 9.0 + 2.0 / 9.0 * t6962 * t3051 * t29002 - t301 * (t128413 + t128066 + t125852 + t128145 + t126111 + t126286 + t128495 + t125898 + t128455 + t126417 + t125950 + t126593 + t125984 + t126028 + t126518 + t126372 + t128101 + t127984 + t127946 + t128538 + t126685 + t126069 + t126471 + t128567 + t126328 + t126645 + t126156 + t126562 + t128032 + t126197 + t128370 + t126245) - 4.0 * t6216 * t44280 * t6217 * t19399 + 2.0 * t6216 * t10683 * t6217 * t19409 - t25459 * t31340 / 9.0 - 4.0 / 27.0 * t98416 + t112520 + t6216 * t2665 * t25465 * t4969 / 9.0 + t128589 / 81.0 - 2.0 * t127919 + t6210 * t31674 / 6.0 + t1466 * t193 * t798 * t127728 * t317 / 6.0;
    (t128599,)
}
