//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1820;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta581(t26411: f64, t6914: f64, t22704: f64, t22705: f64, t5345: f64, t22690: f64, t552: f64, t26447: f64, t90607: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64, t26384: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t5187: f64, t562: f64, t26392: f64, t80670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90759, t90781, t90787, t90789, t90791, t90794) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1820(t26411, t6914, t22704, t22705, t5345, t22690, t552, t26447, t90607, t22751, t26397, t22892, t22893, t26396);
        let (t90797, t90805, t90807, t90809, t90818, t90837) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1821(t22892, t22893, t26384, t26388, t7733, t81186, t5318, t552, t5187, t562, t26392, t80670);
    (t90759, t90781, t90787, t90789, t90791, t90794, t90797, t90805, t90807, t90809, t90818, t90837)
}
