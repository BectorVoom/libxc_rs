//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta483(t1184: f64, t52: f64, t460: f64, t24682: f64, t3548: f64, t7310: f64, t3469: f64, t7320: f64, t2134: f64, t24650: f64, t24655: f64, t24659: f64, t24664: f64, t24670: f64, t24675: f64, t24677: f64, t24681: f64, t3552: f64, t3557: f64, t3562: f64, t3587: f64, t488: f64, t7316: f64, t7321: f64, t7326: f64, t7331: f64, t7345: f64, t2127: f64, t3545: f64, t3475: f64, t2132: f64, t607: f64, t2136: f64, t3535: f64, t7338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24683, t24684, t24685, t24690, t24698, t24699, t24702) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1823(t1184, t52, t460, t24682, t3548, t7310, t3469, t7320, t2134, t24650, t24655, t24659, t24664, t24670, t24675, t24677, t24681, t3552, t3557, t3562, t3587, t488, t7316, t7321, t7326, t7331, t7345);
        let (t24704, t24705, t24706, t24712, t24716) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1824(t2127, t3545, t3475, t460, t7320, t2132, t607, t2136, t3535, t7338);
    (t24683, t24684, t24685, t24690, t24698, t24699, t24702, t24704, t24705, t24706, t24712, t24716)
}
