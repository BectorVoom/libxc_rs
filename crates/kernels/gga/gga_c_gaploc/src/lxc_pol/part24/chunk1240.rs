//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1240/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1240<F: Float>(t15482: F, t20560: F, t34814: F, t10241: F, t9439: F, t20555: F, t10543: F, t1407: F, t1429: F, t2365: F, t2366: F, t25729: F, t10421: F, t20887: F, t10424: F, t30733: F) -> (F, F, F, F, F, F) {
    let t34817 = 0.5680433474654925878e0 * t20560 * t15482 * t34814;
    let t34818 = t9439 * t10241;
    let t34821 = 0.22721733898619703511e0 * t20555 * t15482 * t34818;
    let t34822 = t1407 * t10543;
    let t34823 = 0.51123901271894332902e0 * t34822;
    let t34826 = t1429 * t2365 * t2366 * t25729;
    let t34827 = 0.89376224879626066674e-1 * t34826;
    let t34828 = t10421 * t20887;
    let t34829 = 0.14896037479937677779e-1 * t34828;
    let t34830 = t10424 * t30733;
    (t34817, t34821, t34823, t34827, t34829, t34830)
}
