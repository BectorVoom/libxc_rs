//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2412/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412<F: Float>(t14473: F, t5808: F, t5790: F, t950: F, t4475: F, t49532: F, t4472: F, t5811: F, t959: F, t1589: F, t60848: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F, t68883: F, t68885: F) -> (F, F, F, F, F, F) {
    let t68887 = F::cast_from(0.17544670867903938621e1_f64) * t14473 * t5808;
    let t68888 = t5790 * t950;
    let t68891 = F::cast_from(0.31168546390226634766e3_f64) * t49532 * t4475 * t68888;
    let t68894 = F::cast_from(0.10526802520742363173e2_f64) * t959 * t5811 * t4472;
    let t68896 = F::cast_from(0.17544670867903938621e1_f64) * t60848 * t1589;
    let t68897 = t68767 + t68769 + t68771 - t68773 + t68775 + t68883 + t68885 - t68887 + t68891 - t68894 - t68896;
    (t68887, t68888, t68891, t68894, t68896, t68897)
}
