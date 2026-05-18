//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1083/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1083<F: Float>(t1882: F, t35714: F, t35649: F, t35606: F, t110669: F, t111089: F, t11593: F, t1175: F, t142296: F, t142326: F, t1449: F, t150014: F, t151081: F, t242: F, t2469: F, t2574: F, t2606: F, t27742: F, t28204: F, t28208: F, t28340: F, t28345: F, t28387: F, t33274: F, t33346: F, t33754: F, t33766: F, t33782: F, t35724: F, t3746: F, t3842: F, t3972: F, t3977: F, t446: F, t53797: F, t54032: F, t729: F, t7440: F, t762: F, t98123: F) -> F {
    let t152285 = t1882 * t35714;
    let t152328 = t1882 * t35649;
    let t152334 = t1882 * t35606;
    let t152347 = F::new(2.0) / F::new(9.0) * t152285 - F::new(2.0) / F::new(9.0) * t11593 * t2606 * t33754 * t3746 + F::new(2.0) / F::new(3.0) * t446 * t2574 * t1175 * t33346 + t446 * t729 * t33274 * t3842 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t242 * t151081 + t446 * t729 * t3977 * t33766 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t53797 * t110669 * t28208 + F::new(4.0) / F::new(9.0) * t53797 * t111089 * t28387 - F::new(4.0) / F::new(27.0) * t54032 * t111089 * t28345 + F::new(4.0) / F::new(9.0) * t53797 * t98123 * t28204 - t142296 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t729 * t2469 * t35724 + F::new(2.0) / F::new(3.0) * t446 * t729 * t762 * t27742 * t1449 - F::new(4.0) / F::new(9.0) * t152328 + F::new(2.0) / F::new(3.0) * t446 * t729 * t3977 * t33782 + F::new(2.0) / F::new(3.0) * t152334 - F::new(2.0) / F::new(3.0) * t446 * t2574 * t762 * t7440 * t3972 + t142326 + F::new(2.0) / F::new(3.0) * t446 * t242 * t150014 + F::new(4.0) / F::new(9.0) * t53797 * t110669 * t28340;
    t152347
}
