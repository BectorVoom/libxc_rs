//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1323;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta397(t2495: f64, t9385: f64, t2491: f64, t744: f64, t760: f64, t2492: f64, t2514: f64, t9367: f64, t9371: f64, t200: f64, t631: f64, t202: f64, t635: f64, t2548: f64, t2490: f64, t2595: f64, t39490: f64, t39492: f64, t39495: f64, t39498: f64, t39501: f64, t39506: f64, t39508: f64, t39510: f64, t39512: f64, t39515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39815, t39816, t39818, t39821, t39823, t39825, t39840) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1323(t2495, t9385, t2491, t744, t760, t2492, t2514, t9367, t9371, t200, t631, t202, t635);
        let (t39871, t39875, t39886, t39894, t39909) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1324(t2514, t2492, t2548, t2490, t2595, t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515);
    (t39815, t39816, t39818, t39821, t39823, t39825, t39840, t39871, t39875, t39886, t39894, t39909)
}
