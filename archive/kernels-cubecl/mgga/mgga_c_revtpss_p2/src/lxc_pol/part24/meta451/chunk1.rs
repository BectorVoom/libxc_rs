//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1416/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1416<F: Float>(t1569: F, t2769: F, t786: F, t10985: F, t15017: F, t1580: F, t41117: F, t1565: F, t40781: F, t40488: F, t4354: F, t268: F, t40452: F, t4371: F) -> (F, F, F, F, F, F) {
    let t50208 = t786 * t1569 * t2769;
    let t50214 = t15017 * t10985;
    let t50248 = t41117 * t1580;
    let t50370 = t40781 * t1565;
    let t50372 = t40488 * t4354;
    let t50377 = t40452 * t4371 * t268;
    (t50208, t50214, t50248, t50370, t50372, t50377)
}
