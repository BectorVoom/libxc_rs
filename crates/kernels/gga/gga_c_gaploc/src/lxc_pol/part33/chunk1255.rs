//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1255/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1255<F: Float>(t12161: F, t2089: F, t1445: F, t2087: F, t28381: F, t33081: F, t33084: F, t33090: F, t33092: F, t33095: F, t33098: F, t33101: F, t33105: F, t33109: F, t33112: F, t33114: F, t33117: F, t33126: F, t33127: F, t723: F) -> (F,) {
    let t39027 = t2089 * t12161;
    let t39032 = t33081 - t33084 + t33090 + t33092 - 0.13803453343411469884e2 * t2087 * t1445 * t39027 * t723 + t33095 - t33098 + t33101 - t33105 + t33109 - t33112 - t33114 + t33117 - t33126 - t33127 + t28381;
    (t39032,)
}
