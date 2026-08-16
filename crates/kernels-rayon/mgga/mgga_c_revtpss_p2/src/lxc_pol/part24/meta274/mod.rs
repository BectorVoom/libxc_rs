//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta274(t18531: f64, t2484: f64, t5819: f64, t750: f64, t2611: f64, t5825: f64, t706: f64, t4305: f64, t4311: f64, t5941: f64, t72: f64, t757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18532, t18539, t18540, t18544, t18545, t18547, t18555, t18556) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1047(t18531, t2484, t5819, t750, t2611, t5825, t706, t4305, t4311, t5941, t72, t757);
    (t18532, t18539, t18540, t18544, t18545, t18547, t18555, t18556)
}
