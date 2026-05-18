//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1245/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1245<F: Float>(t33071: F, t10811: F, t7751: F, t326: F, t32893: F, t825: F, t10906: F, t2013: F, t28357: F, t28361: F, t11025: F, t2087: F, t4614: F) -> (F, F, F, F, F, F, F) {
    let t33072 = F::new(0.59584149919750711116e-1) * t33071;
    let t33074 = F::new(0.42900587942220512003e1) * t10811 * t7751;
    let t33077 = F::new(0.92023022289409799224e1) * t825 * t326 * t32893;
    let t33079 = F::new(0.18404604457881959845e2) * t2013 * t10906;
    let t33080 = F::new(0.63904876589867916128e-1) * t28357;
    let t33081 = F::new(0.15976219147466979032e0) * t28361;
    let t33084 = F::new(0.18404604457881959845e2) * t2087 * t4614 * t11025;
    (t33072, t33074, t33077, t33079, t33080, t33081, t33084)
}
