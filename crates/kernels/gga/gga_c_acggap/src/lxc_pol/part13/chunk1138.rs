//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1138/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1138<F: Float>(t7839: F, t8962: F, t31426: F, t31429: F, t35581: F, t35586: F, t35587: F, t35591: F, t35595: F, t35597: F, t35599: F, t35602: F, t35603: F, t35609: F, t35611: F, t35614: F, t35617: F, t35621: F) -> F {
    let t35623 = t7839 * t8962;
    let t35624 = F::new(0.62896184579208304136e-3) * t35623;
    let t35625 = t35581 - t35586 + F::new(0.42874018118069736972e-3) * t35587 - F::new(0.32155513588552302729e-2) * t35591 + t35595 + t35597 + F::new(0.64311027177104605458e-2) * t35599 + t35602 + t35603 - F::new(0.84046875e-1) * t31426 - F::new(11.0) / F::new(96.0) * t31429 + t35609 + t35611 - F::new(0.47172138434406228102e-3) * t35614 - t35617 - F::new(0.7862023072401038017e-3) * t35621 + t35624;
    t35625
}
