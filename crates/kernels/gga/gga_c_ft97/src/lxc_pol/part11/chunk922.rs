//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 922/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk922<F: Float>(t1882: F, t8555: F, t1863: F, t8232: F, t487: F, t7763: F, t100: F, t38477: F, t103: F, t110: F, t11987: F, t1588: F, t1755: F, t1852: F, t1853: F, t1871: F, t1901: F, t379: F, t38662: F, t38937: F, t432: F, t446: F, t447: F, t452: F, t488: F, t492: F, t499: F, t7751: F, t7966: F, t8183: F, t83: F, t8411: F, t8466: F, t8562: F) -> F {
    let t39013 = t1882 * t8555;
    let t39019 = t8232 * t1863;
    let t39021 = t487 * t7763;
    let t39026 = t38477 * t100;
    let t39047 = -F::new(8.0) * t446 * t452 * t8466 * t8562 + F::new(8.0) * t446 * t83 * t38662 - F::new(4.0) * t446 * t452 * t1852 * t1755 * t1853 - F::new(4.0) / F::new(3.0) * t39013 - F::new(8.0) / F::new(3.0) * t446 * t447 * t499 * t7966 - F::new(8.0) / F::new(27.0) * t39019 + F::new(40.0) / F::new(81.0) * t1901 * t11987 * t39021 * t38937 + F::new(8.0) / F::new(3.0) * t1901 * t39026 * t103 * t7751 * t379 + F::new(8.0) / F::new(3.0) * t446 * t1871 * t110 * t432 * t8183 + F::new(8.0) * t446 * t8411 * t488 * t7751 * t492 - F::new(12.0) * t446 * t8411 * t110 * t1588 * t1755;
    t39047
}
