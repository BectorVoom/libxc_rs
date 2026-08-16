//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta308(t1020: f64, t10510: f64, t2928: f64, t320: f64, t10294: f64, t268: f64, t271: f64, t6546: f64, t2394: f64, t885: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10511, t10523, t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1334(t1020, t10510, t2928, t320, t10294, t268, t271, t6546, t2394, t885);
    (t10511, t10523, t10542, t10544, t10545, t10556)
}
