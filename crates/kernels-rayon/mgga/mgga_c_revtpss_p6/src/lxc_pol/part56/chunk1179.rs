//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1179/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1179(t105364: f64, t33494: f64, t1042: f64, t105236: f64, t105460: f64, t1203: f64, t124590: f64, t124611: f64, t124612: f64, t124632: f64, t124645: f64, t124684: f64, t124711: f64, t124755: f64, t124802: f64, t124862: f64, t124893: f64, t124898: f64, t1769: f64, t1789: f64, t31993: f64, t32015: f64, t33433: f64, t33498: f64, t34899: f64, t34952: f64, t34991: f64, t3719: f64, t5236: f64, t5401: f64, t5405: f64, t5406: f64) -> f64 {
    let t131518 = t105364 * t33494;
    let t131552 = 0.24791552806034007214e-3_f64 * t124632 - 0.56468933516960933998e-3_f64 * t124711 * t32015 * t124612 * t105236 - 0.5578099381357651623e-3_f64 * t131518 * t33498 + 0.56468933516960933998e-3_f64 * t124755 * t32015 * t124612 * t5236 + 0.15058382271189582399e-2_f64 * t34991 * t33433 - 0.11156198762715303246e-2_f64 * t124684 * t31993 * t3719 * t1769 * t1203 - 0.112937867033921868e-2_f64 * t124862 * t32015 * t124645 * t105460 + 0.12395776403017003607e-3_f64 * t124590 * t34952 + 0.11156198762715303246e-2_f64 * t124802 * t1042 * t1789 * t5405 - 0.18822977838986977999e-3_f64 * t124611 * t124898 * t5401 + 0.7437465841810202164e-3_f64 * t124893 * t1042 * t34899 * t5405 - 0.18822977838986977999e-3_f64 * t124611 * t124898 * t5406;
    t131552
}
