//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1170/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1170<F: Float>(t20773: F, t34503: F, t3712: F, t34465: F, t3714: F, t11447: F, t33490: F, t11452: F, t34484: F, t34486: F, t34489: F, t34492: F, t34495: F, t34497: F, t34499: F, t34501: F) -> (F, F) {
    let t34505 = t34503 * t3712 * t20773;
    let t34507 = t34465 * t3714;
    let t34509 = t11447 * t33490;
    let t34510 = t34509 * t11452;
    let t34512 = -F::new(0.53810508162887008105e-7) * t34484 + F::new(0.6629778687778673199e-7) * t34486 - F::new(0.2318836277704281739e-4) * t34489 + F::new(0.2318836277704281739e-4) * t34492 + F::new(0.78584976712469872988e-8) * t34495 - F::new(0.21103240995305505364e-7) * t34497 - F::new(0.70344136651018351214e-8) * t34499 - F::new(0.64087860648527174258e-6) * t34501 + F::new(0.2209926229259557733e-7) * t34505 - F::new(0.64087860648527174258e-6) * t34507 - F::new(0.98332751566569010432e-7) * t34510;
    (t34509, t34512)
}
