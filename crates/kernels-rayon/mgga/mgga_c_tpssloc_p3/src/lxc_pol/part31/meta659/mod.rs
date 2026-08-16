//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1943;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta659(t23127: f64, t5628: f64, t16985: f64, t6621: f64, t1516: f64, t87321: f64, t25068: f64, t4261: f64, t5624: f64, t23133: f64, t87340: f64, t16673: f64, t6620: f64, t849: f64, t23083: f64, t28375: f64, t28396: f64, t81835: f64, t58853: f64, t6605: f64, t828: f64, t9972: f64, t4250: f64, t87199: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1943(t23127, t5628, t16985, t6621, t1516, t87321, t25068, t4261, t5624, t23133, t87340, t16673, t6620);
        let (t98833, t98836, t98838, t98842, t98844) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1944(t849, t98832, t23083, t28375, t28396, t81835, t58853, t6605, t828, t9972, t4250, t87199);
    (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844)
}
