//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1649;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta453(t24098: f64, t24164: f64, t533: f64, t1390: f64, t2095: f64, t23857: f64, t532: f64, t7216: f64, t6879: f64, t193: f64, t201: f64, t2056: f64, t2047: f64, t2591: f64, t23042: f64, t23044: f64, t23049: f64, t23051: f64, t23054: f64, t23057: f64, t23059: f64, t23063: f64, t23067: f64, t23070: f64, t23073: f64, t23081: f64, t23084: f64, t23087: f64, t23090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24165, t24166, t24167, t24169, t24175, t24176, t24191) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1649(t24098, t24164, t533, t1390, t2095, t23857, t532, t7216, t6879, t193, t201, t2056);
        let (t24200, t24217) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1650(t2047, t2591, t23042, t23044, t23049, t23051, t23054, t23057, t23059, t23063, t23067, t23070, t23073, t23081, t23084, t23087, t23090);
    (t24165, t24166, t24167, t24169, t24175, t24176, t24191, t24200, t24217)
}
