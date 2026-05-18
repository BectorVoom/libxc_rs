//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 510/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk510<F: Float>(t251: F, t2718: F, t822: F, t860: F, t213: F, t234: F, t2646: F, t2724: F, t2754: F, t2760: F, t2776: F, t2780: F, t2787: F, t2791: F, t2796: F, t2802: F, t2806: F, t2810: F, t820: F, t837: F, t879: F) -> (F, F, F) {
    let t2811 = t2718 * t251;
    let t2815 = t822 * t860;
    let t2828 = t2776 - t2780 + F::new(0.10975748638225852664e-1) * t2787 - F::new(0.10975748638225852664e-1) * t2791 + t2796 - F::new(0.19514881078765566038e-1) * t2802 + F::new(0.19514881078765566038e-1) * t2806 - t2810 + F::new(0.13170898365871023197e1) * t820 * t2811 * t2724 - F::new(0.13170898365871023197e1) * t820 * t2815 * t837 - F::new(0.65854491829355115987e0) * t820 * t879 * t2646 - F::new(0.65854491829355115987e0) * t820 * t879 * t2754 + F::new(0.65854491829355115987e0) * t213 * t234 * t2760;
    (t2811, t2815, t2828)
}
