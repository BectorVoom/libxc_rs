//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1739;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1740;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta345(t40: f64, t2244: f64, t4195: f64, t12939: f64, t2371: f64, t4199: f64, t1409: f64, t2517: f64, t707: f64, t3966: f64, t75: f64, t12606: f64, t1430: f64, t2250: f64, t4104: f64, t607: f64, t767: f64, zeta_threshold: f64, t52: f64, t78: f64, t1431: f64, t4111: f64, t771: f64, t1484: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12940, t12942, t12943, t12944, t12945, t12946, t12947, t12958) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1739(t40, t2244, t4195, t12939, t2371, t4199, t1409, t2517, t707, t3966, t75, t12606, t1430, t2250, t4104, t607, t767, zeta_threshold);
        let t12971 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1740(t52, t3966, t78, t12606, t1431, t2244, t2250, t4111, t607, t771, t12958, zeta_threshold);
        let t12984 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1741(t1484, t212);
    (t12940, t12942, t12943, t12944, t12945, t12946, t12947, t12971, t12984)
}
