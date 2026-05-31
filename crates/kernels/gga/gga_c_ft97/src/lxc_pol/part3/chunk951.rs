//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 951/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk951<F: Float>(t18690: F, t3891: F, t14098: F, t18514: F, t14081: F, t14080: F, t18497: F, t3892: F, t11593: F, t18643: F, t18648: F, t18652: F, t18656: F, t18660: F, t18664: F, t18668: F, t18672: F, t18677: F, t18682: F, t18687: F, t1901: F, t3281: F, t446: F) -> F {
    let t18691 = t3891 * t18690;
    let t18694 = t14098 * t18514;
    let t18695 = t3891 * t18694;
    let t18698 = t14081 * t18514;
    let t18699 = t14080 * t18698;
    let t18702 = t3892 * t18497;
    let t18703 = t3891 * t18702;
    let t18706 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18643 + t446 * t18648 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18652 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t18656 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t18660 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18664 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3281 * t18668 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t18672 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t18677 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18682 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18687 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t18691 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t18695 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t18699 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11593 * t18703;
    t18706
}
