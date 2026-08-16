//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 628/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk628<F: Float>(t11613: F, t313: F, t3650: F, t773: F, t1645: F, t2963: F, t11777: F, t11781: F, t11785: F, t11788: F, t11792: F, t11795: F, t11798: F, t1966: F, t1991: F, t2087: F, t2639: F, t3025: F, t5640: F, t5974: F, t813: F, t833: F, t9858: F, t9873: F) -> (F, F, F) {
    let t11801 = t313 * t11613;
    let t11804 = t773 * t3650;
    let t11807 = t1645 * t2963;
    let t11811 = F::cast_from(0.51123901271894332902e0_f64) * t1991 * t11777 + F::cast_from(0.15337170381568299871e1_f64) * t5640 * t11781 - F::cast_from(0.51123901271894332902e0_f64) * t1966 * t11785 - F::cast_from(0.62115540045351614476e2_f64) * t2087 * t11788 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t11792 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t11795 - F::cast_from(0.10725146985555128001e1_f64) * t11798 * t2639 + F::cast_from(0.42900587942220512003e1_f64) * t11801 * t9858 + F::cast_from(0.10725146985555128001e1_f64) * t11804 * t5974 - F::cast_from(0.21450293971110256002e1_f64) * t3025 * t11807 - F::cast_from(0.31952438294933958063e-1_f64) * t9873;
    (t11801, t11807, t11811)
}
