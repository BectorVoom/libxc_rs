//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 968/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk968<F: Float>(t16152: F, t29033: F, t11941: F, t9770: F, t325: F, t33643: F, t11991: F, t33653: F, t33660: F, t33671: F, t33674: F, t33680: F, t33682: F, t33687: F, t33690: F, t11742: F, t129: F, t15805: F) -> (F, F, F) {
    let t33692 = t29033 * t16152;
    let t33694 = t9770 * t11941;
    let t33696 = t325 * t33643;
    let t33697 = t33696 * t11991;
    let t33699 = -0.51491428373437201896e-5 * t33653 + 0.25340269868817520618e-3 * t33660 - 0.2613929515635525739e-10 * t33671 + 0.50595483470764842601e-7 * t33674 - 0.12228868272569444445e-4 * t33680 + 0.11594181388521408695e-4 * t33682 - 0.12228868272569444445e-4 * t33687 + 0.73661838332293000031e-9 * t33690 + 0.2845640240200497334e-7 * t33692 - 0.90037598882461338974e-7 * t33694 + 0.21642471925239962898e-3 * t33697;
    let t33701 = t15805 * t129 * t11742;
    (t33696, t33699, t33701)
}
