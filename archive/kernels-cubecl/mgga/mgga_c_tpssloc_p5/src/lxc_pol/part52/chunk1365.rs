//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1365/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1365<F: Float>(t115903: F, t119891: F, t115833: F, t119883: F, t119879: F, t25994: F, t7266: F, t119795: F, t119796: F, t1458: F, t1869: F, t2314: F, t27858: F, t31829: F, t31913: F, t33740: F, t33756: F, t4028: F, t4034: F, t4072: F, t650: F, t6515: F, t652: F, t6862: F, t7983: F, t8103: F, t8682: F) -> (F, F, F, F) {
    let t121102 = t115903 * t119891;
    let t121105 = t115833 * t119883;
    let t121108 = t115833 * t119879;
    let t122875 = t7266 * t25994;
    let t122889 = -F::cast_from(2.0_f64) * t1458 * t31829 * t652 - F::cast_from(2.0_f64) * t4072 * t652 * t8682 - t1869 * t27858 - F::cast_from(2.0_f64) * t2314 * t33740 - F::cast_from(2.0_f64) * t31913 * t4028 - F::cast_from(2.0_f64) * t33740 * t4034 - t33756 * t650 - t6515 * t8103 - t6862 * t7983 + t119795 - t119796 - F::cast_from(2.0_f64) * t122875;
    (t121102, t121105, t121108, t122889)
}
