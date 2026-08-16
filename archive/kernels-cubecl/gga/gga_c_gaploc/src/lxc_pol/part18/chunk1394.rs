//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1394/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1394<F: Float>(t32033: F, t6716: F, t6717: F, t10533: F, t20796: F, t1397: F, t8237: F, t9287: F, t26673: F, t544: F, t26629: F, t3394: F, t4130: F) -> (F, F, F, F, F, F) {
    let t34675 = F::cast_from(0.12423108009070322895e3_f64) * t6716 * t6717 * t32033;
    let t34678 = F::cast_from(0.55213813373645879534e2_f64) * t20796 * t10533 * t32033;
    let t34680 = t1397 * t8237 * t9287;
    let t34681 = F::cast_from(0.29792074959875355558e-1_f64) * t34680;
    let t34683 = t544 * t26673 * t9287;
    let t34684 = F::cast_from(0.14896037479937677779e-1_f64) * t34683;
    let t34686 = t544 * t26629 * t9287;
    let t34687 = F::cast_from(0.29792074959875355558e-1_f64) * t34686;
    let t34688 = t4130 * t3394;
    (t34675, t34678, t34681, t34684, t34687, t34688)
}
