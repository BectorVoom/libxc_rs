//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1789;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta451(t23069: f64, t805: f64, t2628: f64, t2633: f64, t6605: f64, t243: f64, t598: f64, t213: f64, t1894: f64, t236: f64, t2379: f64, t6584: f64, t6604: f64, t6606: f64, t2679: f64, t815: f64, t2684: f64, t23043: f64, t23044: f64, t23049: f64, t23051: f64, t23054: f64, t23057: f64, t23059: f64, t23063: f64, t23067: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23070, t23071, t23072, t23073, t23075, t23076, t23077, t23080, t23081, t23083) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1789(t23069, t805, t2628, t2633, t6605, t243, t598, t213, t1894, t236, t2379, t6584, t6604);
        let (t23084, t23086, t23089, t23092) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1790(t23083, t6606, t2679, t815, t6605, t2684, t23043, t23044, t23049, t23051, t23054, t23057, t23059, t23063, t23067, t23071, t23073, t23081);
    (t23070, t23072, t23075, t23076, t23077, t23080, t23083, t23084, t23086, t23089, t23092)
}
