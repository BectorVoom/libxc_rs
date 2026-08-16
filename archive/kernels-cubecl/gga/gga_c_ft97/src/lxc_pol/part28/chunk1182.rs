//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1182/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1182<F: Float>(t23405: F, t34975: F, t1349: F, t35015: F, t376: F, t35234: F, t1389: F, t139600: F, t147993: F, t148943: F, t1557: F, t1570: F, t1642: F, t26823: F, t27417: F, t27420: F, t27426: F, t27428: F, t3188: F, t32714: F, t32743: F, t35028: F, t5766: F, t5772: F, t6580: F, t7313: F) -> F {
    let t149460 = t23405 * t34975;
    let t149479 = t1349 * t376 * t35015;
    let t149483 = t23405 * t35234;
    let t149491 = t139600 / F::cast_from(9.0_f64) + t149460 / F::cast_from(54.0_f64) - t5766 * t35028 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5772 * t27420 * t1389 * t1570 * t3188 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5772 * t27426 * t1389 * t1557 * t3188 - t6580 * t32743 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t147993 + F::cast_from(4.0_f64) * t148943 + t149479 / F::cast_from(9.0_f64) - t32714 * t26823 / F::cast_from(18.0_f64) + t149483 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5772 * t1642 * t7313 * t27428 + t32714 * t27417 / F::cast_from(9.0_f64);
    t149491
}
