//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 782/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk782<F: Float>(t492: F, t7211: F, t452: F, t488: F, t432: F, t7281: F, t379: F, t7229: F, t8557: F, t1901: F, t32591: F, t32594: F, t32599: F, t32603: F, t32607: F, t32610: F, t32613: F, t32617: F, t32622: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t32625 = t7211 * t492;
    let t32627 = t452 * t488 * t32625;
    let t32630 = t7281 * t432;
    let t32632 = t452 * t488 * t32630;
    let t32635 = t7229 * t379;
    let t32636 = t8557 * t32635;
    let t32639 = F::new(2.0) / F::new(3.0) * t446 * t32591 + F::new(2.0) / F::new(9.0) * t1901 * t32594 + t1901 * t32599 / F::new(9.0) - F::new(4.0) / F::new(3.0) * t1901 * t32603 - F::new(4.0) / F::new(3.0) * t1901 * t32607 + F::new(2.0) / F::new(9.0) * t1901 * t32610 + F::new(4.0) / F::new(3.0) * t446 * t32613 + F::new(2.0) / F::new(3.0) * t446 * t32617 + F::new(2.0) / F::new(3.0) * t446 * t32622 + t446 * t32627 / F::new(3.0) + t446 * t32632 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t1901 * t32636;
    (t32625, t32627, t32630, t32632, t32635, t32636, t32639)
}
