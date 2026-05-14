//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1056/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1056<F: Float>(t11604: F, t26759: F, t11326: F, t27420: F, t11308: F, t11325: F, t2993: F, t20773: F, t3712: F, t34465: F, t3714: F, t11447: F, t33490: F, t11452: F, t11522: F, t21778: F, t8677: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34497 = t11604 * t26759;
    let t34499 = t11326 * t27420;
    let t34501 = t11326 * t11308;
    let t34503 = t2993 * t11325;
    let t34505 = t34503 * t3712 * t20773;
    let t34507 = t34465 * t3714;
    let t34509 = t11447 * t33490;
    let t34510 = t34509 * t11452;
    let t34515 = t21778 * t11522 * t8677;
    (t34497, t34499, t34501, t34503, t34505, t34507, t34509, t34510, t34515)
}
