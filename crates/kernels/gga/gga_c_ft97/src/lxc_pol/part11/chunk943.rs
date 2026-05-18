//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 943/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk943<F: Float>(t178: F, t2280: F, t2282: F, t2296: F, t8640: F, t12116: F, t12122: F, t1643: F, t2265: F, t2266: F, t2281: F, t2294: F, t3613: F, t3621: F, t37315: F, t37320: F, t39575: F, t39603: F, t39604: F, t39606: F, t39608: F, t39613: F, t631: F, t637: F, t643: F, t7966: F, t8654: F, t8671: F, t8680: F) -> F {
    let t39616 = F::new(1.0) / t2280 / t178;
    let t39617 = t2282 * t2282;
    let t39622 = t8640 * t2296;
    let t39624 = F::new(8.0) * t2265 * t12116 * t39575 - F::new(4.0) / F::new(3.0) * t2265 * t12122 * t39575 - F::new(8.0) * t2265 * t2266 * t7966 * t643 + F::new(6.0) * t2265 * t3621 * t37315 - F::new(2.0) * t2265 * t3613 * t37320 - F::new(2.0) / F::new(3.0) * t2265 * t8654 * t1643 * t2294 + F::new(12.0) * t2265 * t8680 * t643 * t8671 - t39603 - F::new(4.0) / F::new(3.0) * t39604 - F::new(160.0) / F::new(27.0) * t39606 - F::new(9.0) / F::new(2.0) * t631 * t637 * t2281 * t39608 - F::new(16.0) * t39613 - F::new(30.0) * t631 * t637 * t39616 * t39617 + F::new(10.0) / F::new(3.0) * t39622;
    t39624
}
