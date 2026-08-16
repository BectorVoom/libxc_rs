//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2111;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta675(t24716: f64, t4997: f64, t15492: f64, t7339: f64, t15734: f64, t7345: f64, t25588: f64, t461: f64, t7324: f64, t1244: f64, t1742: f64, t3068: f64, sigma2: f64, t1210: f64, t24721: f64, t27691: f64, t27700: f64, t86261: f64, t15418: f64, t2121: f64, t4724: f64, t24720: f64, t27710: f64, t24722: f64, t11588: f64, t4729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95542, t95545, t95550, t95556, t95566) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2111(t24716, t4997, t15492, t7339, t15734, t7345, t25588, t461, t7324, t1244, t1742, t3068, sigma2);
        let (t95571, t95573, t95587, t95588, t95590, t95593) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112(t1210, t24721, t27691, t27700, t86261, t15418, t2121, t4724, t24720, t27710, t24722, t11588, t4729);
    (t95542, t95545, t95550, t95556, t95566, t95571, t95573, t95587, t95588, t95590, t95593)
}
