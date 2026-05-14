//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1122/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1122<F: Float>(t10820: F, t16036: F, t6111: F, t1457: F, t2103: F, t32223: F, t32219: F, t11065: F, t5666: F, t28659: F, t10828: F, t2013: F, t10897: F, t10896: F, t1391: F, t825: F) -> (F, F, F, F, F, F, F, F) {
    let t33412 = 0.57200783922960682671e1 * t6111 * t16036 * t10820;
    let t33416 = 0.71500979903700853338e0 * t2103 * t1457 * t32223;
    let t33419 = 0.14300195980740170668e1 * t2103 * t1457 * t32219;
    let t33421 = 0.2556195063594716645e1 * t5666 * t11065;
    let t33429 = 0.12780975317973583226e0 * t28659;
    let t33452 = t2013 * t10828;
    let t33453 = 0.38342925953920749676e0 * t33452;
    let t33454 = t2013 * t10897;
    let t33455 = 0.85206502119823888168e-1 * t33454;
    let t33457 = t825 * t1391 * t10896;
    (t33412, t33416, t33419, t33421, t33429, t33453, t33455, t33457)
}
