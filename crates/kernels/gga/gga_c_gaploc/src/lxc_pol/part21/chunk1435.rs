//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1435/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1435<F: Float>(t12223: F, t1835: F, t12153: F, t12162: F, t12163: F, t12241: F, t1457: F, t1628: F, t2004: F, t2197: F, t28726: F, t33560: F, t33564: F, t33567: F, t33569: F, t33572: F, t33574: F, t33581: F, t33584: F, t33586: F, t5666: F, t833: F) -> (F, F) {
    let t39166 = t12223 * t1835;
    let t39170 = F::new(0.46011511144704899612e1) * t2197 * t12163 + F::new(0.61348681526273199482e1) * t2197 * t12153 + F::new(0.61348681526273199482e1) * t833 * t1628 * t12162 - t33560 + t33564 + t33567 + t33569 + t33572 + t33574 + t33581 - t33584 + t33586 + F::new(0.51123901271894332905e0) * t5666 * t12241 + F::new(0.35750489951850426669e0) * t2004 * t1457 * t39166 - t28726;
    (t39166, t39170)
}
