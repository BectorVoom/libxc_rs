//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1394/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1394(t32033: f64, t6716: f64, t6717: f64, t10533: f64, t20796: f64, t1397: f64, t8237: f64, t9287: f64, t26673: f64, t544: f64, t26629: f64, t3394: f64, t4130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34675 = 0.12423108009070322895e3_f64 * t6716 * t6717 * t32033;
    let t34678 = 0.55213813373645879534e2_f64 * t20796 * t10533 * t32033;
    let t34680 = t1397 * t8237 * t9287;
    let t34681 = 0.29792074959875355558e-1_f64 * t34680;
    let t34683 = t544 * t26673 * t9287;
    let t34684 = 0.14896037479937677779e-1_f64 * t34683;
    let t34686 = t544 * t26629 * t9287;
    let t34687 = 0.29792074959875355558e-1_f64 * t34686;
    let t34688 = t4130 * t3394;
    (t34675, t34678, t34681, t34684, t34687, t34688)
}
