//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1439/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1439(t3720: f64, t5750: f64, t1445: f64, t1457: f64, t1865: f64, t2103: f64, t28876: f64, t28891: f64, t28915: f64, t28916: f64, t28920: f64, t33763: f64, t33773: f64, t33774: f64, t33786: f64, t33788: f64, t33790: f64, t33799: f64, t33814: f64, t39013: f64, t39058: f64, t5748: f64, t6060: f64) -> f64 {
    let t39282 = t5750 * t3720;
    let t39294 = t28876 - t33763 + 0.27606906686822939767e2_f64 * t5748 * t1445 * t39282 * t1865 - t33773 + t33774 + t28891 + t33786 + t33788 + t33790 - t33799 + 0.42900587942220512003e1_f64 * t2103 * t1457 * t39013 - 0.21450293971110256001e1_f64 * t6060 * t1457 * t39058 - t28915 - 0.38342925953920749677e0_f64 * t28916 + t28920 - t33814;
    t39294
}
