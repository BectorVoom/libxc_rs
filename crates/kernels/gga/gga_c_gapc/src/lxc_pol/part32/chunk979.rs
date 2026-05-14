//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 979/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk979<F: Float>(t3757: F, t9638: F, t11848: F, t11850: F, t869: F, t11854: F, t7553: F, t1078: F, t2387: F, t3756: F, t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F) -> (F,) {
    let t33820 = t3757 * t9638;
    let t33823 = t869 * t11848 * t11850;
    let t33825 = t7553 * t11854;
    let t33828 = t2387 * t3756 * t1078;
    let t33830 = 0.20047434126173032506e-6 * t33801 - 0.10551281119038438161e-7 * t33803 - 0.11049275749843950005e-7 * t33808 - 0.2750785565527147423e-6 * t33810 + 0.20240885416666666668e-4 * t33812 + 0.28960308421505737848e-5 * t33815 - 0.25340269868817520617e-3 * t33818 - 0.17376185052903442709e-3 * t33820 + 0.28960308421505737848e-5 * t33823 - 0.25340269868817520617e-3 * t33825 - 0.17376185052903442709e-3 * t33828;
    (t33830,)
}
