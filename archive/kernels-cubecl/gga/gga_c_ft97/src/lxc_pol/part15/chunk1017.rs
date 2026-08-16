//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1017/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1017<F: Float>(t4551: F, t4589: F, t8418: F, t1852: F, t20395: F, t979: F, t61025: F, t110: F, t1866: F, t1871: F, t1901: F, t4436: F, t4458: F, t446: F, t447: F, t4572: F, t4623: F, t75482: F, t75487: F, t75489: F, t75491: F, t75493: F, t83: F, t85531: F, t85538: F, t8557: F) -> (F, F, F, F) {
    let t85882 = t8418 * t4551 * t4589;
    let t85895 = t1852 * t979 * t20395;
    let t85903 = t61025 * t4551;
    let t85924 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t85895 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t1866 * t110 * t85531 + F::cast_from(4.0_f64) * t446 * t83 * t85903 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t75482 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t75487 + F::cast_from(4.0_f64) * t446 * t1871 * t4623 * t4436 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t75489 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t75491 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t75493 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t447 * t110 * t85538 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t8557 * t4458 * t4572;
    (t85882, t85895, t85903, t85924)
}
