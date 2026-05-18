//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 915/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk915<F: Float>(t12914: F, t1562: F, t4614: F, t12806: F, t4540: F, t4673: F, t3116: F, t7995: F, t1445: F, t597: F, t2787: F, t9127: F) -> (F, F, F, F, F) {
    let t41769 = t1562 * t4614 * t12914;
    let t41773 = F::new(0.14300195980740170667e1) * t4540 * t4673 * t12806;
    let t41774 = t7995 * t3116;
    let t41777 = F::new(0.11502877786176224903e2) * t597 * t1445 * t41774;
    let t41778 = t2787 * t9127;
    (t41769, t41773, t41774, t41777, t41778)
}
