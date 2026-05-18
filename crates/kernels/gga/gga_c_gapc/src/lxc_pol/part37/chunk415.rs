//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 415/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk415<F: Float>(t1254: F, t1257: F, t1261: F, t414: F, t491: F, t2053: F, t2058: F, t2059: F, t2078: F, t2141: F, t257: F, t260: F, t266: F, t738: F, t748: F, t751: F, t758: F) -> (F, F) {
    let t2150 = -F::new(0.15474205398478635379e-1) * t414 + F::new(0.5833205e-2) * t1254 - F::new(0.16123583333333333333e-2) * t1257 + F::new(0.61251011229312867192e-4) * t491 - F::new(0.6735290625e-5) * t1261;
    let t2152 = F::new(0.21272952746160294864e-2) * t414 * t257 + F::new(0.42545905492320589728e-2) * t2053 * t748 + F::new(0.63818858238480884592e-2) * t2058 * t2059 - F::new(0.21272952746160294864e-2) * t738 * t2078 - t2141 * t266 - F::new(2.0) * t751 * t758 - t260 * t2150;
    (t2150, t2152)
}
