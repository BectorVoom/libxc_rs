//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1301;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1302;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta344(t9864: f64, t9866: f64, t3966: f64, t751: f64, t707: f64, t2379: f64, t262: f64, t157: f64, t9897: f64, t2244: f64, t4195: f64, t2371: f64, t4199: f64, t40: f64, t1409: f64, t2517: f64, t75: f64, t12606: f64, t1430: f64, t2250: f64, t4104: f64, t607: f64, t767: f64, zeta_threshold: f64, t52: f64, t78: f64, t1431: f64, t4111: f64, t771: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12927, t12928, t12934, t12935, t12942, t12943) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1301(t9864, t9866, t3966, t751, t707, t2379, t262, t157, t9897, t2244, t4195, t2371, t4199);
        let (t12944, t12947, t12958) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1302(t40, t12943, t1409, t2517, t707, t3966, t75, t12606, t1430, t2244, t2250, t4104, t607, t767, zeta_threshold);
        let t12971 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1303(t52, t3966, t78, t12606, t1431, t2244, t2250, t4111, t607, t771, t12958, zeta_threshold);
    (t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971)
}
