//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1327/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1327(t34680: f64, t26673: f64, t544: f64, t9287: f64, t26629: f64, t3394: f64, t4130: f64, t20535: f64, t6578: f64, t12881: f64, t4382: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t34681 = 0.29792074959875355558e-1_f64 * t34680;
    let t34683 = t544 * t26673 * t9287;
    let t34684 = 0.14896037479937677779e-1_f64 * t34683;
    let t34686 = t544 * t26629 * t9287;
    let t34687 = 0.29792074959875355558e-1_f64 * t34686;
    let t34688 = t4130 * t3394;
    let t34690 = t20535 * t34688 * t6578;
    let t34691 = 0.11502877786176224903e1_f64 * t34690;
    let t34699 = 0.53625734927775640005e1_f64 * t544 * t4382 * t874 * t12881;
    (t34681, t34684, t34687, t34691, t34699)
}
