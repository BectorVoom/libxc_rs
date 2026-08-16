//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1763;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta542(t6892: f64, t81186: f64, t1987: f64, t81144: f64, t9537: f64, t107: f64, t835: f64, t240: f64, t656: f64, t666: f64, t2331: f64, t625: f64, t63: f64, t9365: f64, t2752: f64, t606: f64, t22641: f64, t9523: f64, t22690: f64, t6639: f64, t1887: f64, t23069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81375, t81398, t81437, t81439, t81440, t81442) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1763(t6892, t81186, t1987, t81144, t9537, t107, t835, t240, t656, t666, t2331, t625);
        let (t81446, t81547, t81573, t81575, t81591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1764(t63, t9365, t2752, t606, t22641, t9523, t22690, t6639, t1887, t23069);
    (t81375, t81398, t81437, t81439, t81440, t81442, t81446, t81547, t81573, t81575, t81591)
}
