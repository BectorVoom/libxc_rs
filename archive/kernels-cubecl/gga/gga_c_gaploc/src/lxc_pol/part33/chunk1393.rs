//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1393/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1393<F: Float>(t1359: F, t3689: F, t12051: F, t1415: F, t1429: F, t1457: F, t1572: F, t1617: F, t1646: F, t34720: F, t34726: F, t34730: F, t34733: F, t34737: F, t34740: F, t34743: F, t34746: F, t34749: F, t34752: F, t34762: F, t34766: F, t38399: F, t38447: F, t549: F) -> F {
    let t38674 = t1359 * t3689;
    let t38678 = t34720 - t34726 + t34730 + t34733 - t34737 + F::cast_from(0.46011511144704899612e1_f64) * t1617 * t12051 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t38399 + F::cast_from(0.79445533226334281486e-1_f64) * t1429 * t549 * t38447 - t34740 + t34743 + t34746 - t34749 - t34752 - t34762 + t34766 - F::cast_from(0.71500979903700853338e0_f64) * t1415 * t38674 * t1646;
    t38678
}
