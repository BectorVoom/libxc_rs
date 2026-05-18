//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1263/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1263<F: Float>(t10820: F, t10914: F, t2089: F, t539: F, t16036: F, t6111: F, t1457: F, t2103: F, t32223: F, t32219: F, t11065: F, t5666: F) -> (F, F, F, F, F) {
    let t33409 = F::new(0.28600391961480341335e1) * t10914 * t539 * t2089 * t10820;
    let t33412 = F::new(0.57200783922960682671e1) * t6111 * t16036 * t10820;
    let t33416 = F::new(0.71500979903700853338e0) * t2103 * t1457 * t32223;
    let t33419 = F::new(0.14300195980740170668e1) * t2103 * t1457 * t32219;
    let t33421 = F::new(0.2556195063594716645e1) * t5666 * t11065;
    (t33409, t33412, t33416, t33419, t33421)
}
