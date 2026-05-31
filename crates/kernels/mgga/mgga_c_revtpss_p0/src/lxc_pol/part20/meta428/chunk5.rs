//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1612/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612<F: Float>(t12256: F, t3698: F, t1012: F, t1042: F, t1222: F, t12800: F, t12816: F, t225: F, t3600: F, t3604: F, t3620: F, t3647: F, t3692: F, t39443: F, t39449: F, t44283: F, t44286: F, t44289: F, t44291: F, t44293: F, t44321: F, t44326: F, t44333: F, t44343: F, t44346: F, t480: F, t484: F) -> F {
    let t44348 = t3698 * t12256;
    let t44353 = -F::cast_from(0.17149607247227894789e-2_f64) * t44283 - F::cast_from(0.19055119163586549765e-2_f64) * t44286 - F::cast_from(0.22866142996303859719e-2_f64) * t44289 + F::cast_from(0.2540682555144873302e-3_f64) * t44291 - F::cast_from(0.28582678745379824648e-3_f64) * t44293 + F::cast_from(0.21437009059034868486e-3_f64) * t44321 * t225 * t480 * t484 + F::cast_from(0.57165357490759649296e-3_f64) * t44326 + F::cast_from(0.14291339372689912324e-2_f64) * t12800 * t3620 + F::cast_from(0.57165357490759649296e-2_f64) * t3647 * t12816 + F::cast_from(0.12862205435420921092e-2_f64) * t3600 * t1042 * t44333 * t3604 - t1222 * t1012 * t3692 * t39449 / F::cast_from(48.0_f64) + t44343 / F::cast_from(108.0_f64) + t44346 / F::cast_from(27.0_f64) + t1222 * t1012 * t44348 * t39443 / F::cast_from(6.0_f64);
    t44353
}
