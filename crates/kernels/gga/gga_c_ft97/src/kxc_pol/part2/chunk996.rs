//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 996/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk996<F: Float>(t15131: F, t296: F, t18: F, t875: F, t2882: F, t2881: F, t14116: F, t4265: F, t10443: F, t4256: F, t10730: F, t10732: F, t11593: F, t15404: F, t15409: F, t15415: F, t15419: F, t15420: F, t15422: F, t15427: F, t15430: F, t15435: F, t1901: F, t446: F) -> F {
    let t15438 = t296 * t15131;
    let t15441 = t18 * t875;
    let t15442 = t2882 * t15441;
    let t15443 = t2881 * t15442;
    let t15446 = t4265 * t14116;
    let t15447 = t2881 * t15446;
    let t15450 = t10443 * t4256;
    let t15453 = F::new(4.0) / F::new(9.0) * t1901 * t15404 - F::new(4.0) / F::new(9.0) * t11593 * t15409 - F::new(2.0) / F::new(9.0) * t10730 - F::new(8.0) / F::new(81.0) * t10732 - F::new(2.0) / F::new(3.0) * t446 * t15415 + t15419 - F::new(4.0) / F::new(27.0) * t15420 - t446 * t15422 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t15427 + F::new(2.0) / F::new(3.0) * t446 * t15430 + t446 * t15435 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t15438 - F::new(4.0) / F::new(9.0) * t11593 * t15443 - F::new(8.0) / F::new(9.0) * t11593 * t15447 + F::new(2.0) / F::new(9.0) * t1901 * t15450;
    t15453
}
