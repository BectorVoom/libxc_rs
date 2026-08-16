//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 790/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk790(t12797: f64, t587: f64, t3421: f64, t995: f64, t2559: f64, t1820: f64, t1017: f64, t5543: f64, t12774: f64, t12775: f64, t12777: f64, t12781: f64, t12785: f64, t12786: f64, t12787: f64, t12788: f64, t12789: f64, t12790: f64, t12791: f64, t12792: f64, t12793: f64, t12796: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12799 = 4.0_f64 / 9.0_f64 * t587 * t12797;
    let t12800 = t3421 * t995;
    let t12801 = t2559 * t12800;
    let t12803 = 8.0_f64 / 9.0_f64 * t1820 * t12801;
    let t12804 = t3421 * t1017;
    let t12805 = t5543 * t12804;
    let t12807 = 4.0_f64 / 9.0_f64 * t587 * t12805;
    let t12808 = t12774 + t12775 + t12777 + t12781 - t12785 - t12786 + t12787 + t12788 - t12789 + t12790 + t12791 - t12792 - t12793 - t12796 + t12799 + t12803 - t12807;
    (t12799, t12800, t12801, t12803, t12804, t12805, t12807, t12808)
}
