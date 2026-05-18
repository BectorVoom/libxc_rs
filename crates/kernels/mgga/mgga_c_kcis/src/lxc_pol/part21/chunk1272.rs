//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1272/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1272<F: Float>(t1142: F, t95335: F, t95336: F, t95338: F, t95340: F, t95343: F, t95346: F, t95349: F, t95352: F, t95354: F, t95356: F, t95358: F, t95386: F, t95431: F, t95432: F, t95434: F, t95436: F, t95438: F, t95440: F, t95442: F, t95444: F, t95446: F, t95448: F, t95450: F, t95477: F) -> F {
    let t95481 = t1142 * (t95335 + t95336 / F::new(48.0) - t95338 / F::new(12.0) + t95340 / F::new(8.0) - t95343 / F::new(32.0) + t95346 / F::new(27.0) - t95349 / F::new(144.0) + t95352 / F::new(8.0) + t95354 / F::new(4.0) + t95356 / F::new(3.0) + t95358 / F::new(72.0) + t95386 + t95431 + t95432 / F::new(128.0) - t95434 / F::new(96.0) + t95436 / F::new(24.0) + t95438 / F::new(64.0) + t95440 / F::new(96.0) + t95442 / F::new(54.0) - t95444 / F::new(288.0) + t95446 / F::new(4.0) - t95448 / F::new(16.0) + t95450 / F::new(128.0) + t95477);
    t95481
}
