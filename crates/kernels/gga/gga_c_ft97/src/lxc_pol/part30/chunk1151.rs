//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1151/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1151<F: Float>(t36161: F, t8392: F, t112680: F, t112746: F, t112888: F, t112920: F, t112987: F, t114531: F, t11593: F, t143612: F, t143653: F, t143718: F, t143720: F, t143722: F, t143753: F, t1508: F, t153550: F, t15460: F, t1901: F, t24886: F, t25271: F, t28496: F, t28516: F, t28520: F, t28524: F, t2862: F, t29056: F, t29071: F, t29154: F, t29222: F, t29399: F, t296: F, t34081: F, t34082: F, t4151: F, t4162: F, t4167: F, t446: F, t57089: F, t72190: F, t7629: F, t99238: F) -> F {
    let t153901 = t8392 * t36161;
    let t153922 = F::new(4.0) * t1901 * t114531 * t7629 * t4162 + F::new(8.0) / F::new(3.0) * t1901 * t72190 * t7629 * t4167 - F::new(4.0) / F::new(9.0) * t1901 * t112680 * t28516 - F::new(2.0) / F::new(9.0) * t1901 * t99238 * t29222 - F::new(4.0) / F::new(9.0) * t1901 * t112987 * t28520 + F::new(4.0) / F::new(27.0) * t1901 * t112746 * t28524 - F::new(4.0) / F::new(3.0) * t1901 * t112920 * t29056 + F::new(2.0) / F::new(27.0) * t143718 + F::new(2.0) / F::new(27.0) * t143720 - t143722 / F::new(27.0) - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t25271 * t29399 - F::new(4.0) / F::new(9.0) * t11593 * t24886 * t29154 + t1901 * t143653 * t4151 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t153901 + F::new(4.0) / F::new(3.0) * t446 * t2862 * t1508 * t28496 - F::new(2.0) / F::new(9.0) * t1901 * t57089 * t34082 + F::new(8.0) * t1901 * t112888 * t34081 * t4162 + F::new(2.0) * t1901 * t29071 * t143612 * t4167 + F::new(4.0) / F::new(3.0) * t446 * t296 * t153550 - F::new(4.0) / F::new(9.0) * t143753;
    t153922
}
