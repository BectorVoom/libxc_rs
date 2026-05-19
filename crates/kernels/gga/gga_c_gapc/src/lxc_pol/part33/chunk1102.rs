//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1102/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1102<F: Float>(t16152: F, t29033: F, t11941: F, t9770: F, t325: F, t33643: F, t11991: F, t33653: F, t33660: F, t33671: F, t33674: F, t33680: F, t33682: F, t33687: F, t33690: F) -> (F, F) {
    let t33692 = t29033 * t16152;
    let t33694 = t9770 * t11941;
    let t33696 = t325 * t33643;
    let t33697 = t33696 * t11991;
    let t33699 = -F::cast_from(0.51491428373437201896e-5_f64) * t33653 + F::cast_from(0.25340269868817520618e-3_f64) * t33660 - F::cast_from(0.2613929515635525739e-10_f64) * t33671 + F::cast_from(0.50595483470764842601e-7_f64) * t33674 - F::cast_from(0.12228868272569444445e-4_f64) * t33680 + F::cast_from(0.11594181388521408695e-4_f64) * t33682 - F::cast_from(0.12228868272569444445e-4_f64) * t33687 + F::cast_from(0.73661838332293000031e-9_f64) * t33690 + F::cast_from(0.2845640240200497334e-7_f64) * t33692 - F::cast_from(0.90037598882461338974e-7_f64) * t33694 + F::cast_from(0.21642471925239962898e-3_f64) * t33697;
    (t33696, t33699)
}
