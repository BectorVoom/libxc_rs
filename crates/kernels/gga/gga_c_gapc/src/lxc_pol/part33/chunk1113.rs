//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1113/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1113<F: Float>(t11854: F, t7553: F, t1078: F, t2387: F, t3756: F, t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F, t33820: F, t33823: F) -> F {
    let t33825 = t7553 * t11854;
    let t33828 = t2387 * t3756 * t1078;
    let t33830 = F::new(0.20047434126173032506e-6) * t33801 - F::new(0.10551281119038438161e-7) * t33803 - F::new(0.11049275749843950005e-7) * t33808 - F::new(0.2750785565527147423e-6) * t33810 + F::new(0.20240885416666666668e-4) * t33812 + F::new(0.28960308421505737848e-5) * t33815 - F::new(0.25340269868817520617e-3) * t33818 - F::new(0.17376185052903442709e-3) * t33820 + F::new(0.28960308421505737848e-5) * t33823 - F::new(0.25340269868817520617e-3) * t33825 - F::new(0.17376185052903442709e-3) * t33828;
    t33830
}
