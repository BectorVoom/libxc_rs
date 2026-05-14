//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1255/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1255<F: Float>(t1858: F, t3720: F, t12213: F, t1865: F, t38907: F, t739: F, t12176: F, t12219: F, t12223: F, t12244: F, t1445: F, t1991: F, t2009: F, t2021: F, t2087: F, t33060: F, t33067: F, t33069: F, t33072: F, t33074: F, t33077: F, t33079: F, t33080: F, t4614: F, t5662: F, t590: F, t773: F, t813: F, t833: F) -> (F, F) {
    let t39002 = t1858 * t3720;
    let t39013 = t12213 * t1865;
    let t39022 = t739 * t38907;
    let t39026 = -0.71500979903700853338e0 * t2021 * t39002 * t2009 - 0.71500979903700853338e0 * t773 * t12176 * t2009 - 0.92023022289409799224e1 * t813 * t1445 * t12223 * t1865 + 0.43710935587469654631e2 * t833 * t1445 * t39013 - 0.18404604457881959845e2 * t2087 * t4614 * t12219 - t33060 - t33067 + t33069 - 0.51123901271894332905e0 * t5662 * t12244 + 0.2044956050875773316e1 * t1991 * t39022 * t590 + t33072 + t33074 - t33077 - t33079 - t33080;
    (t39013, t39026)
}
