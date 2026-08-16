//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta570(t16465: f64, t225: f64, t12250: f64, t1824: f64, t1799: f64, t3791: f64, t3850: f64, t16028: f64, t1372: f64, t5286: f64, t3879: f64, t16205: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53866, t54014, t54068, t54153, t54165, t54258, t54825, t54840, t54854, t54883) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1987(t16465, t225, t12250, t1824, t1799, t3791, t3850, t16028, t1372, t5286, t3879, t16205, t562);
    (t53866, t54014, t54068, t54153, t54165, t54258, t54825, t54840, t54854, t54883)
}
