//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1007/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1007<F: Float>(t11981: F, t447: F, t1445: F, t11987: F, t1457: F, t10617: F, t10620: F, t1450: F, t1456: F, t9549: F, t9553: F, t9556: F, t9560: F, t9564: F, t9568: F, t9571: F, t9575: F, t9577: F, t9579: F, t9582: F, t9584: F) -> (F, F, F, F) {
    let t12134 = t11981 * t447;
    let t12135 = t1445 * t12134;
    let t12138 = t1457 * t11987;
    let t12145 = -t10617 + t10620 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t12135 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t12138 + F::cast_from(0.85206502119823888171e-1_f64) * t9549 - F::cast_from(0.51123901271894332903e0_f64) * t9553 + F::cast_from(0.51123901271894332903e0_f64) * t9556 + t9560 - t9564 - F::cast_from(0.38342925953920749677e0_f64) * t9568 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    (t12134, t12135, t12138, t12145)
}
