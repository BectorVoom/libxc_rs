//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta656(t23133: f64, t5628: f64, t23041: f64, t5614: f64, t1512: f64, t87261: f64, t16944: f64, t25119: f64, t841: f64, t23083: f64, t28372: f64, t28395: f64, t81782: f64, t81783: f64, t5587: f64, t81803: f64, t87295: f64, t23097: f64, t232: f64, t67793: f64, t815: f64, t2628: f64, t5585: f64, t776: f64, t13228: f64, t4233: f64, t6605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98733, t98736, t98738, t98744, t98746, t98750) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1938(t23133, t5628, t23041, t5614, t1512, t87261, t16944, t25119, t841, t23083, t28372, t28395, t81782, t81783);
        let (t98752, t98754, t98758, t98762, t98766) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939(t5587, t81803, t1512, t87295, t23097, t232, t67793, t815, t2628, t5585, t776, t13228, t4233, t6605);
    (t98733, t98736, t98738, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766)
}
