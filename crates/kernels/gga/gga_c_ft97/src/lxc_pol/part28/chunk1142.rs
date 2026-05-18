//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1142/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1142<F: Float>(t148475: F, t446: F, t9049: F, t1369: F, t147944: F, t2112: F, t28: F, t1039: F, t32869: F, t586: F, t5890: F, t139361: F, t148435: F, t148439: F, t148443: F, t148446: F, t148449: F, t148454: F, t148457: F, t148460: F, t148464: F, t148467: F, t148470: F, t148473: F) -> (F, F, F, F) {
    let t148477 = t446 * t9049 * t148475;
    let t148481 = t1369 * t28 * t2112 * t147944;
    let t148486 = t5890 * t28 * t586 * t32869 * t1039;
    let t148488 = -t148435 / F::new(3.0) - F::new(2.0) * t148439 + t148443 - F::new(2.0) / F::new(3.0) * t148446 - F::new(2.0) / F::new(9.0) * t148449 - F::new(2.0) / F::new(3.0) * t148454 - F::new(8.0) / F::new(9.0) * t148457 + t148460 / F::new(18.0) - F::new(8.0) / F::new(9.0) * t139361 - F::new(8.0) / F::new(9.0) * t148464 + F::new(2.0) / F::new(3.0) * t148467 - F::new(4.0) / F::new(9.0) * t148470 - F::new(2.0) / F::new(9.0) * t148473 + F::new(2.0) / F::new(27.0) * t148477 + F::new(2.0) / F::new(3.0) * t148481 + t148486 / F::new(12.0);
    (t148477, t148481, t148486, t148488)
}
