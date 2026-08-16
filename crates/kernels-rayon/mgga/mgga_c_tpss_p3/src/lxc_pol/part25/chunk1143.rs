//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1143/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1143(t1080: f64, t5178: f64, t3001: f64, t5177: f64, t4180: f64, t4184: f64, t5161: f64, t9176: f64, t11873: f64, t11938: f64, t12145: f64, t12146: f64, t15239: f64, t15241: f64, t15243: f64, t15251: f64, t15259: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64, t9221: f64, t9477: f64) -> (f64, f64, f64, f64, f64) {
    let t15760 = t5178 * t1080;
    let t15763 = t5177 * t3001;
    let t15764 = t15763 * t1080;
    let t15767 = t4184 * t4180;
    let t15770 = t5161 * t9176;
    let t15771 = t15770 * t1080;
    let t15788 = -t9477 + 0.76103703703703703703e-2_f64 * t9221 + 0.1522074074074074074e-1_f64 * t11938 + 0.761037037037037037e-2_f64 * t11873 - t12145 - t12146 + 0.3805185185185185185e-2_f64 * t15239 + 0.19025925925925925925e-1_f64 * t15259 - 0.68493333333333333331e-1_f64 * t15264 - 0.2283111111111111111e-1_f64 * t15268 - 0.11415555555555555555e-1_f64 * t15241 + 0.10274e0_f64 * t15273 + 0.68493333333333333332e-1_f64 * t15277 - 0.57077777777777777777e-2_f64 * t15243 - 0.11415555555555555555e-1_f64 * t15283 + 0.34246666666666666666e-1_f64 * t15288 + 0.17123333333333333333e-1_f64 * t15251;
    (t15760, t15764, t15767, t15771, t15788)
}
