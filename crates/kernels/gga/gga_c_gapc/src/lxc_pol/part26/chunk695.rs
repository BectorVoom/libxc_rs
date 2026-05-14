//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 695/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk695<F: Float>(t1625: F, t198: F, t8820: F, t1622: F, t1043: F, t1674: F, t8778: F, t8782: F, t8790: F, t8794: F, t8796: F, t8799: F, t8802: F, t8805: F, t8811: F, t8815: F, t8818: F) -> (F, F) {
    let t8822 = t8820 * t198 * t1625;
    let t8823 = t1622 * t8822;
    let t8825 = t1043 * t1674;
    let t8827 = -0.56275309320814680968e-8 * t8778 - 0.11255061864162936194e-7 * t8782 + 0.82068159426188076412e-9 * t8790 - 0.5627530932081468097e-7 * t8794 + 0.44316806090141561263e-6 * t8796 - 0.51585700210746790888e-5 * t8799 + 0.12163329537032409896e-2 * t8802 - 0.20241536458333333334e-4 * t8805 - 0.41193142698749761516e-5 * t8811 + 0.67471788194444444446e-5 * t8815 + 0.13900948042322754167e-2 * t8818 + 0.1374296967252737644e-6 * t8823 - 0.33816362383187442026e-4 * t8825;
    (t8822, t8827)
}
