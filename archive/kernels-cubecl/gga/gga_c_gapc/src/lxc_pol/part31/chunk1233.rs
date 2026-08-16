//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1233/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1233<F: Float>(t34465: F, t3714: F, t11447: F, t33490: F, t11452: F, t11522: F, t21778: F, t8677: F, t11523: F, t26226: F, t19670: F, t8681: F) -> (F, F, F, F, F, F) {
    let t34507 = t34465 * t3714;
    let t34509 = t11447 * t33490;
    let t34510 = t34509 * t11452;
    let t34515 = t21778 * t11522 * t8677;
    let t34517 = t11523 * t26226;
    let t34520 = t19670 * t11522 * t8681;
    (t34507, t34509, t34510, t34515, t34517, t34520)
}
