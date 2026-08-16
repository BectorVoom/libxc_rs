//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1424/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1424(t1858: f64, t3720: f64, t12213: f64, t1865: f64, t38907: f64, t739: f64, t12176: f64, t12219: f64, t12223: f64, t12244: f64, t1445: f64, t1991: f64, t2009: f64, t2021: f64, t2087: f64, t33060: f64, t33067: f64, t33069: f64, t33072: f64, t33074: f64, t33077: f64, t33079: f64, t33080: f64, t4614: f64, t5662: f64, t590: f64, t773: f64, t813: f64, t833: f64) -> (f64, f64) {
    let t39002 = t1858 * t3720;
    let t39013 = t12213 * t1865;
    let t39022 = t739 * t38907;
    let t39026 = -0.71500979903700853338e0_f64 * t2021 * t39002 * t2009 - 0.71500979903700853338e0_f64 * t773 * t12176 * t2009 - 0.92023022289409799224e1_f64 * t813 * t1445 * t12223 * t1865 + 0.43710935587469654631e2_f64 * t833 * t1445 * t39013 - 0.18404604457881959845e2_f64 * t2087 * t4614 * t12219 - t33060 - t33067 + t33069 - 0.51123901271894332905e0_f64 * t5662 * t12244 + 0.2044956050875773316e1_f64 * t1991 * t39022 * t590 + t33072 + t33074 - t33077 - t33079 - t33080;
    (t39013, t39026)
}
