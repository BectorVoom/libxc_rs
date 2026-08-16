//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1103/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1103(t16152: f64, t29033: f64, t11941: f64, t9770: f64, t325: f64, t33643: f64, t11991: f64, t33653: f64, t33660: f64, t33671: f64, t33674: f64, t33680: f64, t33682: f64, t33687: f64, t33690: f64) -> (f64, f64) {
    let t33692 = t29033 * t16152;
    let t33694 = t9770 * t11941;
    let t33696 = t325 * t33643;
    let t33697 = t33696 * t11991;
    let t33699 = -0.51491428373437201896e-5_f64 * t33653 + 0.25340269868817520618e-3_f64 * t33660 - 0.2613929515635525739e-10_f64 * t33671 + 0.50595483470764842601e-7_f64 * t33674 - 0.12228868272569444445e-4_f64 * t33680 + 0.11594181388521408695e-4_f64 * t33682 - 0.12228868272569444445e-4_f64 * t33687 + 0.73661838332293000031e-9_f64 * t33690 + 0.2845640240200497334e-7_f64 * t33692 - 0.90037598882461338974e-7_f64 * t33694 + 0.21642471925239962898e-3_f64 * t33697;
    (t33696, t33699)
}
