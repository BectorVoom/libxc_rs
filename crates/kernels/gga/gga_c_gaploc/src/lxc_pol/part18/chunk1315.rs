//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1315/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1315<F: Float>(t33580: F, t10984: F, t11033: F, t1445: F, t1457: F, t2004: F, t2005: F, t2178: F, t28715: F, t32180: F, t32230: F, t33544: F, t33546: F, t33560: F, t33564: F, t33567: F, t33569: F, t33572: F, t33574: F, t33576: F, t5703: F, t833: F) -> F {
    let t33581 = F::cast_from(0.85206502119823888168e-1_f64) * t33580;
    let t33582 = -t33544 - t33546 + t28715 + F::cast_from(0.43710935587469654631e2_f64) * t833 * t1445 * t32230 + F::cast_from(0.46011511144704899612e1_f64) * t2178 * t11033 + F::cast_from(0.71500979903700853338e0_f64) * t2004 * t1457 * t32180 + F::cast_from(0.71500979903700853338e0_f64) * t5703 * t10984 - t33560 + t33564 + t33567 + t33569 + t33572 + t33574 + F::cast_from(0.21450293971110256002e1_f64) * t33576 * t2005 + t33581;
    t33582
}
