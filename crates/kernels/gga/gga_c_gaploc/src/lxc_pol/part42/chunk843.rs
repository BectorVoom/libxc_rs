//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 843/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk843<F: Float>(t14365: F, t14366: F, t14369: F, t14370: F, t14377: F, t14378: F, t1589: F, t1628: F, t2049: F, t2194: F, t2197: F, t45648: F, t45653: F, t45658: F, t45663: F, t45667: F, t45678: F, t45680: F, t45684: F, t45688: F, t47381: F, t47389: F, t49989: F, t50111: F, t50182: F, t5241: F, t5640: F, t568: F, t590: F, t7572: F, t7573: F, t797: F, t808: F, t813: F, t833: F) -> (F,) {
    let t50239 = 0.59584149919750711116e-1 * t47381 - 0.10224780254378866581e1 * t47389 + 0.13803453343411469884e2 * t7572 * t7573 * t50111 + t45648 + t45653 + t45658 + 0.30674340763136599742e1 * t5640 * t5241 * t49989 * t590 + t45663 - t45667 + 0.30674340763136599741e1 * t833 * t1628 * t14369 - 0.23833659967900284446e0 * t797 * t1589 * t14377 - 0.30674340763136599741e1 * t813 * t1628 * t14365 - 0.23005755572352449806e1 * t2194 * t14366 - 0.23005755572352449806e1 * t813 * t568 * t808 * t50182 + 0.23005755572352449806e1 * t2197 * t14370 - 0.35750489951850426669e0 * t2049 * t14378 - 0.10427226235956374445e0 * t45678 + t45680 + t45684 + t45688;
    (t50239,)
}
