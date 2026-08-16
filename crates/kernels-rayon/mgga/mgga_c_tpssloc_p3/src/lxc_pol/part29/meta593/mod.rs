//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2020;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta593(t2332: f64, t81442: f64, t22470: f64, t2358: f64, t63: f64, t9365: f64, t193: f64, t201: f64, t6665: f64, t23285: f64, t2752: f64, t10143: f64, t606: f64, t23020: f64, t6562: f64, t794: f64, t22641: f64, t9523: f64, t22690: f64, t6639: f64, t1887: f64, t23069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81443, t81445, t81446, t81483, t81525, t81539) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2020(t2332, t81442, t22470, t2358, t63, t9365, t193, t201, t6665, t23285, t2752, t10143);
        let (t81547, t81571, t81573, t81575, t81591) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2021(t2752, t606, t23020, t6562, t794, t22641, t9523, t22690, t6639, t1887, t23069);
    (t81443, t81445, t81446, t81483, t81525, t81539, t81547, t81571, t81573, t81575, t81591)
}
