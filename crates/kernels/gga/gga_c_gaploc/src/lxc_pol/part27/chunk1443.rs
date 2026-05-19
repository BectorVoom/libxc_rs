//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1443/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1443<F: Float>(t3720: F, t5750: F, t1445: F, t1457: F, t1865: F, t2103: F, t28876: F, t28891: F, t28915: F, t28916: F, t28920: F, t33763: F, t33773: F, t33774: F, t33786: F, t33788: F, t33790: F, t33799: F, t33814: F, t39013: F, t39058: F, t5748: F, t6060: F) -> F {
    let t39282 = t5750 * t3720;
    let t39294 = t28876 - t33763 + F::cast_from(0.27606906686822939767e2_f64) * t5748 * t1445 * t39282 * t1865 - t33773 + t33774 + t28891 + t33786 + t33788 + t33790 - t33799 + F::cast_from(0.42900587942220512003e1_f64) * t2103 * t1457 * t39013 - F::cast_from(0.21450293971110256001e1_f64) * t6060 * t1457 * t39058 - t28915 - F::cast_from(0.38342925953920749677e0_f64) * t28916 + t28920 - t33814;
    t39294
}
