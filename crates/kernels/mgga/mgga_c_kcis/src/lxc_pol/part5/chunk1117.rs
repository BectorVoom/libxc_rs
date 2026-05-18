//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1117/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1117<F: Float>(t10218: F, t13710: F, t13714: F, t13717: F, t13781: F, t18828: F, t18830: F, t18833: F, t18835: F, t18853: F, t9691: F, t10199: F, t10202: F, t1036: F, t13747: F, t13750: F, t1670: F, t18685: F, t18803: F, t18808: F, t18817: F, t18824: F, t245: F, t3078: F, t3081: F, t4625: F, t4647: F, t4654: F, t6320: F, t6338: F, t934: F) -> F {
    let t18854 = F::new(0.14865e-1) * t18828 - F::new(0.1982e-1) * t18830 - F::new(0.991e-2) * t18833 + F::new(0.1982e-1) * t18835 - t10218 - F::new(0.18344444444444444444e-2) * t9691 - F::new(0.36688888888888888888e-2) * t13710 + t13781 - F::new(0.55033333333333333332e-2) * t13714 + F::new(0.55033333333333333332e-2) * t13717 + t18853;
    let t18857 = F::new(3.0) / F::new(16.0) * t10199 * t18803 - t10202 * t6320 / F::new(8.0) - t3078 * t18808 / F::new(4.0) - t13747 * t4647 / F::new(4.0) + t13750 * t1670 / F::new(2.0) + t4654 * t4625 / F::new(2.0) - t3078 * t18817 / F::new(8.0) + t3081 * t6338 / F::new(4.0) + t1036 * t18685 / F::new(4.0) + t18824 * t934 / F::new(4.0) + t245 * t18854 / F::new(2.0);
    t18857
}
