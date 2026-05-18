//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1078/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1078<F: Float>(t35610: F, t8392: F, t1882: F, t35645: F, t35574: F, t35739: F, t10007: F, t110401: F, t110751: F, t1168: F, t14127: F, t14159: F, t14175: F, t142009: F, t142020: F, t142030: F, t150924: F, t150928: F, t151405: F, t1901: F, t242: F, t2469: F, t2574: F, t265: F, t27986: F, t28128: F, t28136: F, t28368: F, t33452: F, t33716: F, t35516: F, t35594: F, t35634: F, t35737: F, t3870: F, t4005: F, t446: F, t684: F, t729: F, t7440: F, t762: F, t766: F) -> F {
    let t151976 = t8392 * t35610;
    let t151985 = t1882 * t35645;
    let t152028 = t1882 * t35574;
    let t152030 = t1882 * t35739;
    let t152032 = -F::new(4.0) / F::new(3.0) * t1901 * t110751 * t28368 - F::new(4.0) / F::new(3.0) * t1901 * t110401 * t28136 + F::new(2.0) / F::new(3.0) * t142009 + F::new(4.0) / F::new(9.0) * t151976 + F::new(2.0) / F::new(3.0) * t446 * t2574 * t4005 * t7440 - F::new(2.0) * t446 * t242 * t151405 - F::new(2.0) / F::new(9.0) * t151985 + F::new(2.0) / F::new(3.0) * t446 * t2574 * t265 * t150928 + t446 * t729 * t2469 * t35594 / F::new(3.0) + t446 * t729 * t762 * t33452 * t1168 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t2574 * t265 * t150924 + t446 * t729 * t762 * t35516 * t766 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t1901 * t14175 * t35737 * t684 - t1901 * t10007 * t35634 * t684 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t142020 + t1901 * t142030 * t3870 / F::new(9.0) + t1901 * t14159 * t33716 / F::new(9.0) - F::new(4.0) / F::new(3.0) * t1901 * t14127 * t28128 * t27986 - F::new(4.0) / F::new(9.0) * t152028 - F::new(4.0) / F::new(9.0) * t152030;
    t152032
}
