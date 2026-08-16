//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta475(t3014: f64, t343: f64, t6734: f64, t1004: f64, t6758: f64, t1036: f64, t6750: f64, t1940: f64, t3087: f64, t354: f64, t6759: f64, t3: f64, t6740: f64, t23476: f64, t1046: f64, t1935: f64, t23533: f64, t23537: f64, t23541: f64, t23544: f64, t3043: f64, t3134: f64, t3153: f64, t378: f64, t6717: f64, t6747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844(t3014, t343, t6734, t1004, t6758, t1036, t6750, t1940, t3087, t354, t6759, t3, t6740);
        let (t23563, t23564, t23569) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845(t23476, t343, t23562, t1046, t1935, t23533, t23537, t23541, t23544, t23548, t23551, t23554, t23557, t23560, t3043, t3134, t3153, t378, t6717, t6747);
    (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562, t23563, t23564, t23569)
}
