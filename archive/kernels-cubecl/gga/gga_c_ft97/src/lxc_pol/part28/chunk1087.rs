//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1087/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1087<F: Float>(t1882: F, t34649: F, t34653: F, t103: F, t34482: F, t34671: F, t8392: F, t34686: F, t10969: F, t110: F, t11490: F, t11593: F, t11810: F, t137739: F, t138000: F, t144958: F, t145741: F, t1871: F, t1901: F, t1902: F, t23249: F, t23323: F, t26061: F, t26145: F, t26210: F, t3052: F, t32488: F, t32527: F, t32571: F, t32620: F, t3266: F, t3271: F, t34689: F, t379: F, t446: F, t452: F, t488: F, t492: F, t5710: F, t5722: F, t7229: F, t83: F, t8506: F, t8557: F, t925: F) -> F {
    let t146604 = t1882 * t34649;
    let t146631 = t1882 * t34653;
    let t146637 = t103 * t34482;
    let t146642 = t8392 * t34671;
    let t146644 = t8392 * t34686;
    let t146671 = -t138000 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146604 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t110 * t144958 + t446 * t452 * t488 * t34482 * t492 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t8557 * t32571 * t925 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t8557 * t32620 * t925 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t8557 * t7229 * t3052 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t5710 * t26145 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t146631 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t10969 * t32527 + t1901 * t1902 * t146637 * t379 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t146642 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t146644 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11810 * t32488 * t3266 + F::cast_from(2.0_f64) * t1901 * t11490 * t137739 * t3271 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11490 * t23249 * t26145 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t23323 * t26210 + t1901 * t8506 * t34689 / F::cast_from(9.0_f64) - t446 * t83 * t145741 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t26061 * t5722;
    t146671
}
