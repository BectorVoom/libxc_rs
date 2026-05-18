//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1043/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1043<F: Float>(t2477: F, t486: F, t1415: F, t6823: F, t1559: F, t197: F, t1563: F, t202: F, t2486: F, t4786: F, t1428: F, t4360: F) -> (F, F, F, F, F, F) {
    let t18383 = t486 * t2477;
    let t18482 = t1415 * t6823;
    let t18535 = t1559 * t197;
    let t18540 = F::new(1.0) / t1563 / t202;
    let t18676 = t4786 * t2486;
    let t18736 = t4360 * t1428;
    (t18383, t18482, t18535, t18540, t18676, t18736)
}
