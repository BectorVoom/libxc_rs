//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1077/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1077<F: Float>(t1882: F, t35703: F, t35694: F, t35717: F, t35707: F, t13885: F, t14127: F, t141868: F, t142002: F, t151079: F, t151357: F, t151409: F, t1901: F, t242: F, t28267: F, t28349: F, t33759: F, t3837: F, t3842: F, t4005: F, t446: F, t53797: F, t6061: F, t6154: F, t6940: F, t729: F, t7484: F, t762: F, t98123: F) -> (F, F, F, F) {
    let t151897 = t1882 * t35703;
    let t151907 = t1882 * t35694;
    let t151926 = t1882 * t35717;
    let t151954 = t1882 * t35707;
    let t151964 = -t446 * t729 * t4005 * t7484 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t242 * t151409 + F::new(2.0) / F::new(3.0) * t446 * t729 * t6154 * t28267 + F::new(2.0) / F::new(3.0) * t446 * t729 * t762 * t6061 * t6940 + F::new(4.0) / F::new(3.0) * t446 * t242 * t151079 + F::new(4.0) / F::new(9.0) * t53797 * t98123 * t28349 - t142002 - t446 * t242 * t151357 / F::new(3.0) + t151954 / F::new(9.0) + F::new(4.0) / F::new(3.0) * t1901 * t13885 * t33759 * t3837 + F::new(2.0) * t1901 * t14127 * t141868 * t3842;
    (t151897, t151907, t151926, t151964)
}
