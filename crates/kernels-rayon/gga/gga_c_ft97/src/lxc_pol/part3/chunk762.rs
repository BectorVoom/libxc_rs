//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 762/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk762(t15874: f64, t420: f64, t419: f64, t15847: f64, t15850: f64, t15852: f64, t15855: f64, t15858: f64, t15861: f64, t15863: f64, t15866: f64, t15869: f64, t15872: f64) -> (f64, f64) {
    let t15875 = t420 * t15874;
    let t15876 = t419 * t15875;
    let t15878 = -0.51074886703703703704e-1_f64 * t15847 + 0.34049924469135802469e-1_f64 * t15850 + 0.34049924469135802469e-1_f64 * t15852 - 0.42562405586419753087e-2_f64 * t15855 + 0.38306165027777777778e-1_f64 * t15858 - 0.51074886703703703704e-1_f64 * t15861 - 0.17024962234567901235e-1_f64 * t15863 + 0.21281202793209876543e-2_f64 * t15866 + 0.85124811172839506173e-2_f64 * t15869 - 0.12768721675925925926e-1_f64 * t15872 + 0.6384360837962962963e-2_f64 * t15876;
    (t15876, t15878)
}
